use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    space: f32,
    #[serde(rename = "fontSize")]
    font_size: f32,
    #[serde(rename = "charCount")]
    char_count: usize,
    #[serde(rename = "charsCount")]
    chars_count: usize,
}


struct Input {
    rects: Vec<Rect>,
    width: f64,
    line_height: f64,
}

struct InputData {
    no_of_inputs: u32,
    inputs: Vec<Input>,
}

struct Output {
    wrapped_block: Vec<WrappedBlock>,
    raw_rect: RawRect,
}

#[derive(Debug)]
struct WrappedBlock {
    height: f32,
    width: f32,
    rects: Vec<Rect>,
}

#[derive(Debug)]
struct RawRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn wrap_lines_ex(word_rects: Vec<Rect>, width: f32, line_height: f32) -> (Vec<WrappedBlock>, RawRect) {
    let mut wrapped_block: Vec<WrappedBlock> = Vec::new();
    let mut max_width = 0.0;
    let mut lw = 0.0;
    
    let mut n_lrects: Vec<Rect> = Vec::new();
    let mut line_max_font_height = 0.0;
    let mut font_sizes = Vec::new();
    let mut fts = Vec::new();
    let mut y_pos = 0.0;
    let mut x_pos = 0.0;
    let mut line_initial_y = 0.0;

    for word_rect in word_rects.into_iter() {
        let rw = word_rect.width;
        let is_line_break_here = check_line_break(&word_rect, &mut line_initial_y);

        if (lw + rw > width && !n_lrects.is_empty()) || is_line_break_here {
            let last_rect_space = n_lrects.last().unwrap().space;

            wrapped_block.push(WrappedBlock {
                height: line_max_font_height,
                width: lw - last_rect_space,
                rects: n_lrects.clone(),
            });

            if max_width < lw {
                max_width = lw - last_rect_space;
            }
            n_lrects.last_mut().unwrap().space = 0.0;
            n_lrects.clear();

            lw = 0.0;
            line_max_font_height = word_rect.height;

            if !fts.is_empty() {
                let max_font_size = fts.iter().cloned().fold(0.0, f32::max);
                font_sizes.push(max_font_size);
                y_pos += max_font_size * line_height;
                fts = vec![word_rect.font_size];
            } else {
                font_sizes.push(word_rect.font_size);
                y_pos += word_rect.font_size * line_height;
            }

            // Clone `word_rect` before assigning to `new_word_rect`
            let mut new_word_rect = word_rect.clone();
            x_pos = 0.0;
            new_word_rect.x = x_pos;
            new_word_rect.y = y_pos;
            
            // Clone `new_word_rect` before pushing into `n_lrects`
            n_lrects.push(new_word_rect.clone());
            x_pos += new_word_rect.width + new_word_rect.space;
        } else {
            fts.push(word_rect.font_size);
            if line_max_font_height < word_rect.height {
                line_max_font_height = word_rect.height;
            }

            let mut new_word_rect = word_rect.clone();
            new_word_rect.x = x_pos;
            new_word_rect.y = y_pos;
            
            n_lrects.push(new_word_rect.clone());
            x_pos += new_word_rect.width + new_word_rect.space;
        }
        lw += rw + word_rect.space;
    }

    if !n_lrects.is_empty() {
        if max_width < lw {
            max_width = lw;
        }
        wrapped_block.push(WrappedBlock {
            height: line_max_font_height,
            width: lw,
            rects: n_lrects.clone(),
        });

        font_sizes.push(fts.iter().cloned().fold(0.0, f32::max));
    }

    let raw_height: f32 = font_sizes.iter().enumerate().map(|(index, &font_size)| {
        if index == font_sizes.len() - 1 {
            wrapped_block.last().unwrap().height
        } else {
            font_size * line_height
        }
    }).sum();

    let raw_rect = RawRect {
        x: 0.0,
        y: 0.0,
        width: max_width,
        height: raw_height,
    };

    (wrapped_block, raw_rect)
}


fn check_line_break(rect: &Rect, line_initial_y: &mut f32) -> bool {
    let mut is_break = false;
    if rect.y > *line_initial_y {
        is_break = true;
    }
    *line_initial_y = rect.y;
    is_break
}

fn main() {
    let json_data = r#"
     [
        {
            "x": 0,
            "y": 0,
            "width": 113.05555555555554,
            "height": 56,
            "space": 8.222222222222221,
            "fontSize": 37.897488591999995,
            "charCount": 6,
            "charsCount": 6
        },
        {
            "x": 121.27777777777777,
            "y": 0,
            "width": 65.77777777777777,
            "height": 56,
            "space": 8.222222222222221,
            "fontSize": 37.897488591999995,
            "charCount": 5,
            "charsCount": 4
        },
        {
            "x": 195.27777777777777,
            "y": 0,
            "width": 178.83333333333331,
            "height": 56,
            "space": 8.222222222222221,
            "fontSize": 37.897488591999995,
            "charCount": 12,
            "charsCount": 11
        },
        {
            "x": 382.3333333333333,
            "y": 0,
            "width": 90.44444444444443,
            "height": 56,
            "space": 0,
            "fontSize": 37.897488591999995,
            "charCount": 6,
            "charsCount": 6
        }
    ]"#;

    // Deserialize the JSON into a Vec<Rect>
    let word_rects: Vec<Rect> = serde_json::from_str(json_data).expect("Failed to parse JSON");

    // Call the function
    let (wrapped_block, raw_rect) = wrap_lines_ex(word_rects, 283.66666666666663, 1.0);

    // Print the result
    println!("Wrapped Block: {:#?}", wrapped_block);
// println!("Wrapped Block: {:#?}", wrapped_block.last().unwrap().rects);
// for rect in wrapped_block.iter() {
//     println!("Rect: {:#?}", rect.rects);
// }

    println!("Raw Rect: {:#?}", raw_rect);
    // println!("Max Width: {:#?}", raw_rect.width);

}
