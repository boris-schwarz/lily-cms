# Create Single
POST {{target}}/content
Content-Type: application/json
```
{
    "title": "a",
    "body": "b",
    "summary": "c"
}
```
HTTP 200

[Asserts]
jsonpath "$.title" == "a"
jsonpath "$.body" == "b"
jsonpath "$.summary" == "c"



# Read Single
GET {{target}}/content/some-id-here
HTTP 200

[Asserts]
jsonpath "$.id" == "some-id-here"
jsonpath "$.title" contains "Lorem"
jsonpath "$.body" contains "Doloribus"
jsonpath "$.summary" contains "sit amet"



# Update Single
PATCH {{target}}/content/some-id-here
Content-Type: application/json
```
{
    "title": "a",
    "body": "b",
    "summary": "c"
}
```
HTTP 200

[Asserts]
jsonpath "$.id" == "some-id-here"
jsonpath "$.title" == "a"
jsonpath "$.body" == "b"
jsonpath "$.summary" == "c"
