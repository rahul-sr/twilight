let transport = Transport::single_node("https://example.com")?;
let client = Elasticsearch::new(transport);

let response = client
    .search(SearchParts::Index(&["tweets"]))
    .from(0)
    .size(10)
    .body(json!({
        "query": {
            "match": {
                "message": "Elasticsearch rust"
            }
        }
    }))
    .send()
    .await?;

let response_body = response.json::<Value>().await?;
let took = response_body["took"].as_i64().unwrap();
for hit in response_body["hits"]["hits"].as_array().unwrap() {
    // print the source document
    println!("{:?}", hit["_source"]);
}