use super::Twitter;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let ops = vec![
        "Twitter",
        "postTweet",
        "getNewsFeed",
        "follow",
        "postTweet",
        "getNewsFeed",
        "unfollow",
        "getNewsFeed",
    ];
    let params = vec_2d![[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]];
    let results = Vec::from(option_array![
        null,
        null,
        vec![5],
        null,
        null,
        vec![6, 5],
        null,
        vec![5]
    ]);

    test(ops, params, results);
}

#[test]
fn t2() {
    let ops = vec![
        "Twitter",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "postTweet",
        "getNewsFeed",
        "follow",
        "getNewsFeed",
        "unfollow",
        "getNewsFeed",
    ];
    let params = vec_2d![
        [],
        [1, 5],
        [2, 3],
        [1, 101],
        [2, 13],
        [2, 10],
        [1, 2],
        [1, 94],
        [2, 505],
        [1, 333],
        [2, 22],
        [1, 11],
        [1, 205],
        [2, 203],
        [1, 201],
        [2, 213],
        [1, 200],
        [2, 202],
        [1, 204],
        [2, 208],
        [2, 233],
        [1, 222],
        [2, 211],
        [1],
        [1, 2],
        [1],
        [1, 2],
        [1]
    ];
    let results = option_array![
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        null,
        vec![222, 204, 200, 201, 205, 11, 333, 94, 2, 101],
        null,
        vec![211, 222, 233, 208, 204, 202, 200, 213, 201, 203],
        null,
        vec![222, 204, 200, 201, 205, 11, 333, 94, 2, 101]
    ];

    test(ops, params, Vec::from(results));
}

fn test(ops: Vec<&str>, params: Vec<Vec<i32>>, results: Vec<Option<Vec<i32>>>) {
    let mut twitter = Twitter::new();

    for (i, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        call(i, &mut twitter, op, param, result);
    }
}

fn call(i: usize, twitter: &mut Twitter, op: &str, param: Vec<i32>, result: Option<Vec<i32>>) {
    match op {
        "Twitter" => {}
        "postTweet" => twitter.post_tweet(param[0], param[1]),
        "getNewsFeed" => assert_eq!(twitter.get_news_feed(param[0]), result.unwrap(), "{}", i),
        "follow" => twitter.follow(param[0], param[1]),
        "unfollow" => twitter.unfollow(param[0], param[1]),
        _ => todo!(),
    }
}
