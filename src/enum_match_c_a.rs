// Matches Are Exhaustive 
// The arm's patterns must cover all possibilities

// Example - consider this will cause a bug, if we try to compile
// this code, we'll get this error 

// Explanation - Rust knows that we didn't cover every possible case, and even knows 
// which pattern we forgot!
// Matches in Rust are exhausitive: we must exhaust every last possibility in order for the code
// to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicity
// handle the None case, it protects us from assuming that we have a valuewhen we might have a null, thus
// making the billion-dollar mistake  
 
