---
title: "Error Handling in Go: Not ACTUALLY Bad"
date: "2025-03-03"
tags:
  - "golang"
  - "errors"
---

I've been using Go for a few years now - mainly for personal projects, but also at work where I use Typescript and Rust alongside it.
While each of these languages can have their own frustrations, I haven't found Go to be particularly problematic. The language is simple and efficient,
so I'm able to get things done quickly without worrying too much.

However, whenever I recommend Go to others, they often complain about its error handling. While I agree that it can be frustrating at times, 
I don't think it's as bad as some people make it out to be. In fact, I think it can be a good selling point for the language, 
as it aligns with the idea of Go being a ["dumb down language"](https://news.ycombinator.com/item?id=13432199).

## Go-lang error philosophy

If you've worked with Go, you've probably worked with the built-in error type. Go code uses error values to indicate an abnormal state.
For example, the `os.Open` function returns a non-nil error value when it fails to open a file.
(https://go.dev/blog/error-handling-and-go)

```go
func Open(name string) (file *File, err error)
```
Developers can choose to ignore errors in Go, but this flexibility may not be preferred by all team members. However, this can be helpful when doing a proof of concept.
```go
foo, _ := bar()
```
I prefer the error-handling approach in Go over some other languages for various reasons. In certain languages, 
such as those using `try catch` blocks, it is not always apparent how potential errors are dealt with.
This can result in confusing control flow and can make debugging more challenging as the application grows.

In my opinion, the error handling in Go provides significant benefits. 

1. There are no hidden control flows, making the code easier to understand
2. No unexpected exceptions. 
3. Errors are fully controlled as values that can be handled.

It's also important to note that In Go, you're not required to handle every error, but the syntax encourages you to consider errors as important parts of your program flow.
However,I believe that the Rust `Result` type offers a more graceful and controlled approach to handling errors.

## Simplifying repetitive error handling

For most functions that return errors, there will be a boilerplate code like below
```go
err := foo()
if err != nil {
	return err
}
```
One of the common ways I handle errors in Go-lang is by using a global error handler. In a scenario where I have a GRPC server,
My handlers can throw different errors. Based on these errors, I have to return different error codes and messages for my clients. 
A way I have been doing this for some time is by writing an error handler and handling the error in it.
```go
type ErrorHandler struct {
        logger lib.Logger
        env    *lib.Env
}

type GenerateErrorWithGrpcCodesRequest struct {
        Err     error
        Payload []byte
        Method  string
}

func (h *ErrorHandler) GenerateErrorWithGrpcCodes(data GenerateErrorWithGrpcCodesRequest) error {
        if data.Err != nil {
                h.logger.Errorw("Error", "details", data.Err.Error(), "payload", data.Payload, "method", data.Method)

                if errors.Is(data.Err, gorm.ErrRecordNotFound) {
                        return status.Error(codes.NotFound, "Resource not found")
                } else if errors.Is(data.Err, lib.ErrSignUpDissabled) {
                        return status.Error(codes.Unavailable, "Sign-up disabled.")
                } // ...more cases

                return status.Error(codes.Unknown, "Unknown error occurred")
        }
        return nil
}

func (h *ErrorHandler) WithErrorHandler(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
        payload, err := json.Marshal(req)

        if err != nil {
                return nil, err
        }

        resp, err := handler(ctx, req)

        return resp, h.GenerateErrorWithGrpcCodes(GenerateErrorWithGrpcCodesRequest{
                Err:     err,
                Payload: payload,
                Method:  info.FullMethod,
        })
}
```
This way I can use Grpc `UnaryInterceptor` which allows me to handle error as the way I expected. 
```go
func (s *ServeCommand) Run() lib.CommandRunner {
	return func(
        // other imports
		errorHandler *handler.ErrorHandler,
	) {
		s := grpc.NewServer(grpc.UnaryInterceptor(errorHandler.WithErrorHandler))
        // ...
	}
}
```

By doing this I can simplify my errors and I don't have to worry about my errors in my handlers. because of this implementation, I can return my errors as follows

```go
func (s *PurchaseHandler) AdminFindByValue(ctx context.Context, req *proto.FindByValueRequest) (*proto.FindByValueResponse, error) {
	return s.PurchaseService.AdminFindByValue(req)
}
```

I believe writing an interceptor can allow you to handle errors and reduce repetitive error handling. Also, another benefit of doing this is in case you have
a scenario where you want to handle the error in the handler you have the flexibility to do that as well.

In the go.dev blog you can find a similar example where App Engine application with an HTTP handler retrieves a record from the datastore and formats it with a template.

[https://go.dev/blog/error-handling-and-go](https://go.dev/blog/error-handling-and-go)

## The Zen of Go

The Zen of Go mentions two important proverbs:

1. Simplicity matters
2. Plan for failure, not success

[https://dave.cheney.net/2020/02/23/the-zen-of-go](https://dave.cheney.net/2020/02/23/the-zen-of-go)

Go programmers believe that robust programs are composed of pieces that handle the failure cases before the happy path. 
In the space that Go was designed for; server programs, multi-threaded programs, programs that handle input over the network, dealing with unexpected data, 
timeouts, connection failures, and corrupted data must be front and center of the programmer’s mind if they are to produce robust programs.


> "I think that error handling should be explicit, this should be a core value of the language."
> Peter Bourgon, [GoTime #91](https://changelog.com/gotime/91)

I believe that the explicit way errors are handled in programming is beneficial because it compels you to consider the worst-case scenario first. 
I appreciate how Go-lang is assisting us in this regard.

## Exception-based code can be frustrating

There is another method for managing errors known as exceptions in languages such as Javascript and Python. 
However, I prefer the concept of errors as values because I find it easier to understand. To illustrate, 
let's consider an example where we retrieve a record, update its value, and save an audit message.
```js
const updateValue = async (id: string, value: number) => {
    const item = await getFromDB(id);
    item.value = value;
    await save(item)
    item.audit = `price updated ${new Date().toISOString()}`
    await save(item)
}
```
Imagine a scenario where the database fails. The code given in this case does not guarantee proper handling of exceptions. 
What if the first save operation is successful but the second one fails? In such a scenario, the audit message won't be stored in the database.
Although transactions can be used to handle this issue, the code still does not ensure that all exceptions are handled. 
To improve this, we can use a `try-catch` block to handle errors, as shown below:
```js
const updateValue = async (id: string, value: number) => {
    try {
        const item = await getFromDB(id);
        item.value = value;
        await save(item);
        item.audit = `price updated ${new Date().toISOString()}`;
        await save(item);
    } catch (err) {
        // which one errored ?
    }
};
```

Handling errors using try-catch blocks can be improved by having multiple try-catch blocks in the codebase where Exceptions can be thrown. 
This allows us to identify which specific block failed, as shown below:
```js
const updateValue = async (id: string, value: number) => {
    let item = null;
    try {
        item = await getFromDB(id);
    } catch (err) {
        // handle error
    }
    try {
        item.value = value;
        await save(item);
    } catch (err) {
        // handle error
    }
    try {
        item.audit = `price updated ${new Date().toISOString()}`;
        await save(item);
    } catch (err) {
        // handle error
    }
};
```
If you want to handle errors properly and achieve the same behavior as Go, you have to follow certain steps. Honestly, 
I prefer how Go handles errors as it is less confusing and more reliable. However, in Javascript, it can be a bit challenging. 
For instance, if you want to return an item, you have to use `Promise<product | null>` and handle the `null` case separately.

This is one of the reasons why Javascript services can be less reliable compared to Go. 
But, with proper error handling, you can still write reliable services in Javascript. 


Let's take a look at similar examples in Go and see how Go-lang can help us to solve issues like this with error handling.
```go
func (s *ProductServiceImpl) UpdateValue(id string, value int) {
	tx := s.MasterDB.Begin()
	product := model.Product{}

	if err := tx.Omit("id").Where(&model.Product{
		Id: id,
	}).First(&product).Error; err != nil {
		tx.Rollback()
		return nil, err
	}

	product.Value = value

	if err := tx.Save(&product).Error; err != nil {
		tx.Rollback()
		return nil, err
	}

        product.Audit = fmt.Sprintf("price updated %s", 
                time.Now().Format(time.RFC3339))

	if err := tx.Save(&product).Error; err != nil {
		tx.Rollback()
		return nil, err
	}

	tx.Commit()
}
```
In the code example above, we see a similar scenario to what we previously examined. 
Thanks to Golang's error handling, we can use database transactions and handle the situation more elegantly.
As shown, we can retrieve the first event and perform all the updates. In the event of a database failure, we can simply roll back.
We only commit once the entire transaction has been completed. This approach helps us to ensure that worst-case scenarios are addressed 
and that we do not shoot our foot.

I believe this is one of the best examples I can give of why I truly love error handling in Go-lang. especially when working with databases that can fail this can be very helpful.

In functional programming, this is known as the fancy term: [violating referential transparency](https://stackoverflow.com/questions/28992625/exceptions-and-referential-transparency/28993780#28993780).
This [blog post](https://devblogs.microsoft.com/oldnewthing/?p=36693) from Microsoft's engineering blog in 2005 still holds true today, namely:

> My point isn’t that exceptions are bad. My point is that exceptions are too hard and I’m not smart enough to handle them.

You can read more about this [here](https://rauljordan.com/why-go-error-handling-is-awesome/).

## Go's error syntax

One of the biggest complaints about Go-lang is the error syntax

[leave "if err != nil" alone?](https://github.com/golang/go/issues/32825)

> The Go2 proposal [#32437](https://github.com/golang/go/issues/32437) adds new syntax to the language to make the `if err != nil { return ... }` boilerplate less cumbersome.
There are various alternative proposals: [#32804](https://github.com/golang/go/issues/32804) and [#32811](https://github.com/golang/go/issues/32811) as the original one is not universally loved.
To throw another alternative in the mix: Why not keep it as is? 
I've come to like the explicit nature of the if `err != nil` construct and as such I don't understand why we need new syntax for this. Is it really that bad?

  [miekg](https://github.com/miekg) commented on Jun 28, 2019

I agree with miekg that it's not that bad. If you're tired of typing `if err != nil { return ... }` in Go, you can set up a simple remap like [@ThePrimeagen](https://youtu.be/lvKQh3Od6V4?si=ZtSs9VW28FoAR65h&t=713) did,
so you don't have to type it every time. I think it's just something you have to type if you're writing in Go, but it gets the job done, which is the whole point of [Go-lang](https://news.ycombinator.com/item?id=13432199a).

## References

- https://rauljordan.com/why-go-error-handling-is-awesome/
- https://dave.cheney.net/2020/02/23/the-zen-of-go
- https://go.dev/blog/error-handling-and-go
- https://github.com/golang/go/issues/32825
- https://jesseduffield.com/Gos-Shortcomings-1/
- https://news.ycombinator.com/item?id=13432199
