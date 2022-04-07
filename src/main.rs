/**
 * Your task for this reciation is play around with the Rust standard Vector and
 * higher order functions (map, fold, filter) in rust. Notice that these fucntions 
 * can only be appliedto iterators and also return iterators (except for fold). 
 * You can use .collect() to convert the results back into a collection. Also, since
 * these functions can return iterators you can chain them together as well.
 */

fn main() {
	// Vector Practice
	//Problem 1
	// Produce a vector, first_vector, that contains the strings, "programming", "in"
	// "rust", "is", "fun"
	// Hint: use the vec! macro
	
	assert_eq!(first_vector, ["programming", "in", "rust", "is", "fun"]);

	//Problem 2
	// Add the strings: "but", "borrowing", "is", "difficult" to
	// the back of first_vector. Add the string "functional" to the front
	// of first_vector
	// Hint: use push and insert
	
	assert_eq!(first_vector, ["functional", "programming", "in", "rust", "is", "fun", "but", "borrowing", "is", "difficult"]);

	//Problem3
	// Create a new vector, second_vector containing the string: "Learning"
	// Now append all the elements of first_vector to second_vector
	// Hint: use append
	
	assert_eq!(second_vector, ["Learning"]);


	assert_eq!(second_vector, ["Learning", "functional", "programming", "in", "rust", "is", "fun", "but", "borrowing", "is", "difficult"]);
	
	// Remove the last element from first_vector and from second_vector
	// Hint: use pop
	// What is returned in each case?
	

	// Functional Practice

	//Problem 4
	// Produce a vector that contains even numbers in the range (1 to 10)
	// inclusive Hint: use range (1..11) and filter
	

	//Problem 5
	// Use the vector produced in problem 1 and find the sum of all the numbers
	// contained in it Hint: use fold
	

	//Problem 6
	// Repeat problem 2 but now multiply the numbers and assume an initial
	// multiplier of 2
	// Hint: you just need to change one parameter of fold
	


	//Problem 7
	// Take the square of all elements of the vector which was produced in
	// problem 1
	// Hint: use map
	
	

	//Extra Practice
	// Find the sum of all prime numbers in the range (100 to 200) inclusive
	// Hint: define a function is_prime, use filter and then fold
	
}
