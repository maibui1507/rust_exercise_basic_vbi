// Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho. 
use std::io;

fn main() {
    // str cho san
    let text = " byte indexing is fast and fast";

    // nhap vao 1 t bat ky
    let mut word = String::new();
    println!("Print a word");
    io::stdin().read_line(&mut word).expect("Error at input");
    word.pop();

    let mut count = 0;
    let mut i = 0;
    let mut j = 0;

    let first_character = word.chars().next().unwrap().to_lowercase().next().unwrap();

    let txt = text.chars().collect::<Vec<char>>();
    let w = word.chars().collect::<Vec<char>>();
    let mut check = true;

    // dem word trong text

    while i < text.len(){
        if word.len() > text.len(){
            break;
        }
        check = true;
        if txt[i]==first_character{
            while j < word.len(){
                if txt[i+j]!=w[j]{
                    check = false;
                    break;
                }
                j+=1;
            }
            if check{
                count +=1;
            }
        }
        
        j = 0;
        i+=1;
    }
    
    println!("{}",count);
    
 
}

