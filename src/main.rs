use serde::Deserialize; //brings deserialize trait into scope so i use it in the derive macros 

#[derive(Deserialize)] //tells serde to auto-generate JSON parsing code for each struct at compile time
struct ApiResponse {
    data: Vec<Post>, //Vec is equivalent to an array in javascript
}

#[derive(Deserialize)]
struct Post {
    title: String, 
    selftext: Option<String>, //makes this field optional 
    created_utc: f64, 
}

#[tokio::main] //allows async functions to run 
async fn main() -> Result<(),Box<dyn std::error::Error>> { //() means if it working nothing meaningful will be returned. Box<dyn std::error::Error> meains it failed and here is some type of error
    //GET Request 
    let response = reqwest::get("https://api.pullpush.io/reddit/search/submission/?subreddit=curlyhair&title=gel")
        .await?; //? if error sends it to top level rather than dealing with it there
    let api_response = response.json::<ApiResponse>().await?; //the ::<ApiResponse> turbofish tells serde which type to deserialize into

    for res in &api_response.data {
        println!("{}", res.title); 
        println!("{:?}", res.selftext.as_deref().unwrap_or("no body")); 
        println!("{}", res.created_utc)
    }

    Ok(()) //signals success 
}

//think of Result as a container type with two slots, tells rust the two possible outcomes the functiona can return 
//Result < SuccessType, ErrorType >

