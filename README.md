# GO RUST 

A series of patterns implemented in Rust that are common to the go language. While the go examples, will 
compile please note that they may not be strictly idiomatic. 


### Single Producer

A pattern that calls a function from another thread/goroutine, and sends its result on a channel. Caller is free pursue to
other interests.

##### Go example

```go
package main

import (
	"fmt"
	"time"
)

// runs forever
func singleProducer(callme func(chan<-int)) <-chan int {
	intChan := make(chan int)

	go callme(intChan)

	return intChan
}


func main() {

	fib := func(sendto chan<-int) {
		prev := 1
		prev2 := 1

		sendto <- prev
		sendto <- prev2

		for {
			prev, prev2 = prev2, prev + prev2
			sendto <- prev2
		}
	}

	out := singleProducer(fib)

	for {
		fmt.Println(<-out)
		time.Sleep(time.Millisecond * 100)
	}
}

```

 


