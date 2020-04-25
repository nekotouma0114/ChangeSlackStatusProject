extern crate reqwest;

pub async fn get_request(url: String) -> String{
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        return response
}