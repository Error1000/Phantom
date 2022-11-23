fn main(){
       for arg in &["-static",
                    "-Tlink.x",
		    //"-nostartfiles",
		   ] {
               println!("cargo:rustc-link-arg={}", arg);
       }

}
