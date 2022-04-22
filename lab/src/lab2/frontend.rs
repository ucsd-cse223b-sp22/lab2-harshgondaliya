use tribbler::storage::BinStorage;
use tribbler::trib;

pub struct FrontEnd {
    pub bin_storage: Box<dyn BinStorage>,
}
impl trib::Server for FrontEnd {
    /// Creates a user.
    /// Returns error when the username is invalid;
    /// returns error when the user already exists.
    /// Concurrent sign ups on the same user might both succeed with no error.
    async fn sign_up(&self, user: &str) -> TribResult<()> {}

    /// List 20 registered users.  When there are less than 20 users that
    /// signed up the service, all of them needs to be listed.  When there
    /// are more than 20 users that signed up the service, an arbitrary set
    /// of at lest 20 of them needs to be listed.
    /// The result should be sorted in alphabetical order.
    async fn list_users(&self) -> TribResult<Vec<String>>;

    /// Post a tribble.  The clock is the maximum clock value this user has
    /// seen so far by reading tribbles or clock sync.
    /// Returns error when who does not exist;
    /// returns error when post is too long.
    async fn post(&self, who: &str, post: &str, clock: u64) -> TribResult<()>;

    /// List the tribs that a particular user posted.
    /// Returns error when user has not signed up.
    async fn tribs(&self, user: &str) -> TribResult<Vec<Arc<Trib>>>;

    /// Follow someone's timeline.
    /// Returns error when who == whom;
    /// returns error when who is already following whom;
    /// returns error when who is trying to following
    /// more than trib.MaxFollowing users.
    /// returns error when who or whom has not signed up.
    /// Concurrent follows might both succeed without error.
    /// The count of following users might exceed trib.MaxFollowing=2000,
    /// if and only if the 2000'th user is generated by concurrent Follow()
    /// calls.
    async fn follow(&self, who: &str, whom: &str) -> TribResult<()>;

    /// Unfollow someone's timeline.
    /// Returns error when who == whom.
    /// returns error when who is not following whom;
    /// returns error when who or whom has not signed up.
    async fn unfollow(&self, who: &str, whom: &str) -> TribResult<()>;

    /// Returns true when who following whom.
    /// Returns error when who == whom.
    /// Returns error when who or whom has not signed up.
    async fn is_following(&self, who: &str, whom: &str) -> TribResult<bool>;

    /// Returns the list of following users.
    /// Returns error when who has not signed up.
    /// The list have users more than trib.MaxFollowing=2000,
    /// if and only if the 2000'th user is generate d by concurrent Follow()
    /// calls.
    async fn following(&self, who: &str) -> TribResult<Vec<String>>;

    /// List the tribs of someone's following users (including himself).
    /// Returns error when user has not signed up.
    async fn home(&self, user: &str) -> TribResult<Vec<Arc<Trib>>>;
}
