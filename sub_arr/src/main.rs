// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//         let sub_arr = [6,8,10];


fn main() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    let sub_arr_2 = [6,10];
    let sub_arr_3 = [10];

    let len_org_arr = org_arr.len();
    let len_sub_arr = sub_arr_3.len();

    let mut i = 0;
    let mut j = 0;

    while i<len_org_arr && j<len_sub_arr{
        if org_arr[i]==sub_arr_3[j]{
            i+=1;
            j+=1;
            if j==len_sub_arr{
                println!("True");
            }
        }
        else{
            i = i-j+1;
            j=0;
        }
    }
                
}
