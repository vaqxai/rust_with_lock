# rust_with_lock_auto
A simple locking library that allows you to easily and safely access the interior of Arc<Mutex> pointers

Adds the following methods to each Arc<Mutex<T>> (hence the name 'auto'):

```
mutex.with_lock(|data| {
    // do stuff
});

// this is the same but uses try_lock instead of lock().unwrap()
mutex.with_lock_try(|data| {
    // do stuff
});
```
The closure can also return any type

You can also clone the inner value simply by doing:

```
let inner_value_clone = mutex.get_clone(); // if you're sure you can lock the mutex

let inner_value_clone_optional = mutex.try_get_clone(); // if you want an option that's none when the mutex can't be locked (uses try_lock internally)
```
But the inner value of course needs to implement Clone


does not introduce new types and works on all existing Arc<Mutex<T>>es and Weak<Mutex<T>>s