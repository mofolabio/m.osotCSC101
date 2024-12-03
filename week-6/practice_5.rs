fn main(){
	let fullname = "Pan-Atlantic University";
	println!();
	println!("Name: {}", fullname);
	println!();
	println!("Before trim");
	println!("lenght is {}",fullname.len());
	println!();
	println!("After trim");
	println!("lenghth is {}",fullname.trim().len());
}