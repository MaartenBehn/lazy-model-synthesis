use image::{GenericImageView, ImageReader};
use octa_force::glam::{ivec2, IVec2};
use octa_force::log::info;
use octa_force::OctaResult;
use crate::rules::{Rule, RuleReq};
use crate::value::{Value, ValueColor, ValueNr};

pub fn gen_rules_from_image(path: &str, offsets: Vec<IVec2>) -> OctaResult<(Vec<Rule>, Vec<ValueColor>)> {
    let img = ImageReader::open(path)?.decode()?;
    
    let mut value_colors = vec![];
    let mut rules = vec![];
    
    for x in 1..(img.width() -1) {
        for y in 1..(img.height() -1) {
            let pixel = img.get_pixel(x, y);
            let value_color = ValueColor::from_rgba(pixel);
            
            let index = value_colors.iter().position(|c| *c == value_color).unwrap_or_else(|| {
                let i = value_colors.len();
                value_colors.push(value_color);
                rules.push(Rule::new(Value::from_value_nr(i as ValueNr)));
                i
            });
            
            let mut rule_req = RuleReq::new();
            
            for offset in offsets.iter() {
                let req_pos = ivec2(x as i32, y as i32) + *offset;
                if req_pos.x < 0 || req_pos.y < 0 || req_pos.x >= img.width() as i32 || req_pos.y >= img.height() as i32 {
                    unreachable!()
                }

                let req_pixel = img.get_pixel(req_pos.x as u32, req_pos.y as u32);
                let req_value_color = ValueColor::from_rgba(req_pixel);
                let reg_index = value_colors.iter().position(|c| *c == req_value_color).unwrap_or_else(|| {
                    let i = value_colors.len();
                    value_colors.push(req_value_color);
                    rules.push(Rule::new(Value::from_value_nr(i as ValueNr)));
                    i
                });
                
                
                
                let reg_value = Value::from_value_nr(reg_index as ValueNr);
                
                rule_req.reqs.push((*offset, reg_value));
            }
            
            if !rules[index].reqs.contains(&rule_req) {
                rules[index].reqs.push(rule_req);
            }
        }
    }
    
    value_colors.insert(0, ValueColor::new(1, 0, 0));
    
    info!("Color: {value_colors:?}");
    info!("Rules: {rules:?}");
    
    Ok((rules, value_colors))
}

