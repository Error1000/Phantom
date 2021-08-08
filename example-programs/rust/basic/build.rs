fn main(){
	for arg in &["-static",
		     "-Tlink.x"] {
		println!("cargo:rustc-link-arg={}", arg);
	}

}
