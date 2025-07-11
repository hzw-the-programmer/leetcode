// 355. Design Twitter

use core::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque, vec_deque};

struct Tweet {
    id: i32,
    time: usize,
}

impl Tweet {
    fn new(id: i32, time: usize) -> Self {
        Self { id, time }
    }
}

struct HeapItem<'a> {
    tweet: &'a Tweet,
    iter: vec_deque::Iter<'a, Tweet>,
}

impl<'a> HeapItem<'a> {
    fn new(tweet: &'a Tweet, iter: vec_deque::Iter<'a, Tweet>) -> Self {
        Self { tweet, iter }
    }
}

impl<'a> PartialEq for HeapItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tweet.time == other.tweet.time
    }
}

impl<'a> Eq for HeapItem<'a> {}

impl<'a> PartialOrd for HeapItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.tweet.time.partial_cmp(&other.tweet.time)
    }
}

impl<'a> Ord for HeapItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tweet.time.cmp(&other.tweet.time)
    }
}

struct User {
    followees: HashSet<i32>,
    tweets: VecDeque<Tweet>,
}

impl User {
    fn new() -> Self {
        Self {
            followees: HashSet::new(),
            tweets: VecDeque::new(),
        }
    }
}

struct Twitter {
    users: HashMap<i32, User>,
    recent_max: usize,
    time: usize,
}

impl Twitter {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            recent_max: 10,
            time: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let user = self.users.entry(user_id).or_insert_with(|| User::new());

        if user.tweets.len() == self.recent_max {
            user.tweets.pop_back();
        }

        self.time += 1;
        user.tweets.push_front(Tweet::new(tweet_id, self.time));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if let Some(user) = self.users.get(&user_id) {
            let iters = user
                .followees
                .iter()
                .filter(|&&id| id != user_id)
                .map(|id| self.users.get(id).unwrap().tweets.iter())
                .chain(Some(user.tweets.iter()));

            let mut heap = BinaryHeap::new();

            for mut iter in iters {
                if let Some(tweet) = iter.next() {
                    heap.push(HeapItem::new(tweet, iter));
                }
            }

            let mut res = vec![];

            while let Some(mut item) = heap.pop() {
                res.push(item.tweet.id);

                if res.len() == self.recent_max {
                    break;
                }

                if let Some(tweet) = item.iter.next() {
                    heap.push(HeapItem::new(tweet, item.iter));
                }
            }

            res
        } else {
            vec![]
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users.entry(followee_id).or_insert_with(|| User::new());
        self.users
            .entry(follower_id)
            .or_insert_with(|| User::new())
            .followees
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert_with(|| User::new())
            .followees
            .remove(&followee_id);
    }
}

#[cfg(test)]
mod tests;
