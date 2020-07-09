resource "aws_dynamodb_table" "a" {
  name = "${var.stage}-a"
  hash_key = "hash_key"
  range_key = "range_key"

  read_capacity  = 1
  write_capacity = 1

  attribute {
    name = "hash_key"
    type = "S"
  }

  attribute {
    name = "range_key"
    type = "S"
  }
}

resource "aws_dynamodb_table" "b" {
  name = "${var.stage}-b"
  hash_key = "key"

  read_capacity  = 1
  write_capacity = 1

  attribute {
    name = "key"
    type = "S"
  }
}
