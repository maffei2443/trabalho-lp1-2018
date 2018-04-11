fn main(){
    let _not_a_reference = 3;
    match _not_a_reference {
        mut mutt => {mutt+=4; println!("1-match(nao_ref) => ref nova; nova = {}", mutt); },
        ref mut mutt => {*mutt+=4; println!("2-match(nao_ref) => ref nova; nova = {}", mutt); },
        not_mutt  => {println!("3-match(nao_ref) => ref nova; nova = {}", not_mutt); },
    }
}
