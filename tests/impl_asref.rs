use derive_asref::AsRef;

pub struct User {
    pub id: usize,
}

// This should be
// ```
// impl AsRef<User> for UserWithName {
//   fn as_ref(&self) -> &User {
//     &self.inner
//   }
// }
// ```
#[derive(AsRef)]
pub struct UserWithName {
    #[as_ref(target = "User")]
    inner: User,
    _name: String,
}

#[test]
fn should_impl_asref() {
    let anonymous = User { id: 42 };
    let alice = UserWithName {
        inner: anonymous,
        _name: "Alice".to_owned(),
    };

    assert_eq!(alice.as_ref().id, 42);
}
