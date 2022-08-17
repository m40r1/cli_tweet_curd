# cli_tweet_curd
An cli async backend in rust

It uses json formated queries

some example requests

Gets all the tweets
curl http://localhost:8080/tweets

Gets by id
its the /tweets/{id}
curl http://localhost:8080/tweets/1

Creates new tweet
curl -d '{"message":"teste"}' -H "Content-type:applicantion/json" -X POST http://localhost:8080/tweets

Updates tweet 
curl -d '{"message":"testewqeqwe"}' -H "Content-type:applicantion/json" -X PUT h
ttp://localhost:8080/tweets/3
