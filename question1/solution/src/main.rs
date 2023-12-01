use std::fs::read_to_string;
use std::collections::HashMap;
fn main() {
    let mut ans:u32 = 0;
    let v:Vec<String> = vec![String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine"), String::from("ten")];
    let mut hmap = HashMap::new();
    for(index, value) in v.iter().enumerate(){
        hmap.entry(value).or_insert(index+1);
    }
       for line in read_to_string("input.txt").unwrap().lines(){
         let mut mini:i32 = i32::MAX;
        let mut maxi:i32 = i32::MIN;
        let mut startdigit:u32 = 1;
        let mut enddigit:u32 = 1;

        let temp = line.to_string();
        for (_, value) in v.iter().enumerate(){
            let result = temp.find(value);
            match result {
                Some(ind) => {
                     if mini > ind as i32{
                    mini = ind as i32;
                    match hmap.get(value){
                            Some(&val) => {
                                startdigit = val as u32;
                            },
                            None => println!("nothing"),
                        } 
                }
                                       
                },
                None => continue, 
            }
            let res = temp.rfind(value);
            match res {
                Some(ind) =>{
                    if maxi < ind as i32 {
                    maxi = ind as i32;
                     match hmap.get(value){
                            Some(&val) => {
                                enddigit = val as u32;
                            } 
                            None=>println!("nothing"),
                        }    
                }
                

                },
                None => println!("nothing")    
                
            }
        }
        
        for (index, c) in temp.char_indices(){
            let ch = c;
            if c as u8 >= 48 && c as u8 <=57{
                if mini > index as i32{
                    startdigit = ch as u32 - 48 as u32;
                break;
                }
            }
        }
        for (index, c) in temp.char_indices().rev(){
            let ch = c;
            if c as u8 >= 48 && c as u8 <= 57{
                if  index as i32  - 1 > maxi{
                    enddigit = ch as u32 - 48 as u32;
                    break;
                }
            }
        }
        ans += startdigit*10 + enddigit;
    

    }

    println!("{}", ans);
}
