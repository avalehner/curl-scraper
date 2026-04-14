#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> { //() means if it working nothing meaningful will be returned. Box<dyn std::error::Error> meains it failed and here is some type of error
    println!("Hello, world!"); 
    Ok(()) //signals success 
}

//think of result as a container type with two slots, tells rust the two possible outcomes the functiona can return 
//Result < SuccessType, ErrorType >


