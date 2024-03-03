---
title: "Error Handling in Go: Not ACTUALLY Bad"
date: "2024-03-02"
tags:
  - "golang"
  - "errors"
---

I have been using Go for a few years now. Mostly use Go-lang for personal projects and also in my day job I use Typescript, Rust, Go-lang. I believe most of these 
languages have things that can frustrate the developers. But the time I have been developing using Go-lang I didn't feel much frustrated. Part of this can be how simple the language is and 
the language helps me to get things done quickly and I don't have to worry about it much later. 

But everytime I talk to someone and recommend them to golang most of people always complain about error handling. Sometimes error
handling can get frustrated in golang but most of the time I dont think it's that bad. Honestly sometimes I do belive it's one of the
good selling points in golang and it goes well with the idea of Go-lang ["a dumb down language"](https://news.ycombinator.com/item?id=13432199)

## Go-lang error philosophy

If you've worked with Go, you've probably worked with the build-in error type. Go code uses error values to indicate an abnormal state.
For example, the os.Open function returns a non-nil error value when it fails to open a file.
(https://go.dev/blog/error-handling-and-go)
```go
func Open(name string) (file *File, err error)
```
Go allows developers to ignore errors as well, this flexibility might not be preferred by all team members. But I belive this can be helpful when doing a poc
```go
foo, _ := bar()
```

I find the error handling approach in Go to be preferable to some other languages for several reasons. 
In some languages, like those using `try catch` blocks, it's not always clear how potential errors are handled.
This can lead to confusing control flow and difficult to debug when application grows

In my opinion, the error handling in Go offers significant benefits:

1. No hidden control-flows. this can make code easier to understand.
2. No unexpected exceptions. 
3. full control of erors as values you can handle.

It's also important to note that In Go, you're not required to handle every error, but the syntax encourages you to consider errors as important parts of your program flow.
But I belive that the Rust `Result` type offers a more graceful and controlled approach to handling errors.

## Simplifying repetitive error handling

For most of functions that return error there will be a boilerplate code like below
```go
err := foo()
if err != nil {
	return err
}
```
One of the common ways I approach error handling in golang is actually by using a global error handler. In a scenario where I have a grpc server my handlers can throw different 
errors. based on these errors I have to throw different error codes and messages for my clients. A way I have been doing this for some time is by writing a error handler and 
handling the error in this.
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

By doing this I can simplify my errors and I dont have to worry about my errors in my handlers. because of this implementation I am able to return my errors as follows

```go
func (s *PurchaseHandler) AdminFindByValue(ctx context.Context, req *proto.FindByValueRequest) (*proto.FindByValueResponse, error) {
	return s.PurchaseService.AdminFindByValue(req)
}
```

I belive writing an interceptor can allow you to handle errors and reduce the repetitive error handling. Also another benefit of doing this is in case you have
a scenario where you want to handle the error in the handler you have the flexibility to do that as well.

In the go.dev blog you can find a similar example where App Engine application with an HTTP handler that retrieves a record from the datastore and formats it with a template.

[https://go.dev/blog/error-handling-and-go](https://go.dev/blog/error-handling-and-go)

## The Zen of Go

The Zen of Go mentions two important proverbs:

1. Simplicity matters
2. Plan for failure, not success

[https://dave.cheney.net/2020/02/23/the-zen-of-go](https://dave.cheney.net/2020/02/23/the-zen-of-go)

Go programmers belive that robust programs are composed from pieces that handle the failure cases before the happy path. 
In the space that Go was designed for; server programs, multi threaded programs, programs that handle input over the network, dealing with unexpected data, 
timeouts, connection failures and corrupted data must be front and centre of the programmer’s mind if they are to produce robust programs.


> "I think that error handling should be explicit, this should be a core value of the language."
> Peter Bourgon, [GoTime #91](https://changelog.com/gotime/91)

Personally I do belive that explicit way errors are handling is good. because it forces you to think about fail case first. In programming I do belive that
we should always consider the worse case scenario and I really like how go is helping us to do that. 

## Exception-based code can be frustrting

A different way to handle errors would be exceptions in other languages like Javascript , Python. The reason why I belive that the errors as values is good is because I'm not smart enough.
lets take the below example where we fetch for record and update the value and save the audit message.

```js
const updateValue = async (id: string, value: number) => {
    const item = await getFromDB(id);
    item.value = value;
    await save(item)
    item.audit = `price updated ${new Date().toISOString()}`
    await save(item)
}
```

Imagine a scenario the database fails in this case. This code does not ensure exceptions are properly handled. In a case where the first save is a success but second one fails? 
the audit message wont be in the database. we can use transactions here to handle this in a nice way but still my point is code does not ensure expcetions are handled.

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
It's possible to handle the error by using the `try-catch`. but how do you know which one is the error ?
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

if you want the same behaviour as go and if you wanted to handle errors properly this is what you have to do now? at this point honestly I would prefer how go handle errors. 
this does not make it nicer. I belive this is very confusing too. and what if you wanted to return the item here ? now your return statements will be `Promise<product | null>` 
now in the place where you call this function you have to handle the `null` case as well. 

isn't this why Javascript services are not very reliable ? compaired to go-lang you can write your services to be very reliable. In languages like Javascript you can do it 
but I belive it is not that easy

Lets take a look in to similar example in golang and actually see how go-lang can helps us to solve issues like this with error handling. 
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

	product.Audit = "price updated"

	if err := tx.Save(&product).Error; err != nil {
		tx.Rollback()
		return nil, err
	}

	tx.Commit()
}
```

Above code is a similar example to what we looked before. as you can see because of how golang enforce us to handle errors we can easily use database transactions 
and handle this case in a more nicer way. as you can see we can fetch for the first event and do all the update. in case our databse fails we can just rollback. 
we only commit onece the complete transaction is done this can help us to ensure worse case senarios are handled and we dont shoot our own foot. 

I belive this is one of the best examples I can give why I truely love error handling in golang. specially when working with databases which can fail this can be very helpful.


In functional programming,this is known as the fancy term: [violating referential transparency](https://stackoverflow.com/questions/28992625/exceptions-and-referential-transparency/28993780#28993780).
This [blog post](https://devblogs.microsoft.com/oldnewthing/?p=36693) from Microsoft's engineering blog in 2005 still holds true today, namely:

> My point isn’t that exceptions are bad. My point is that exceptions are too hard and I’m not smart enough to handle them.

You can read more about this [here](https://rauljordan.com/why-go-error-handling-is-awesome/).

## Go's error syntax

One of the biggest complain about golang is the error syntax

[leave "if err != nil" alone?](https://github.com/golang/go/issues/32825)

> The Go2 proposal [#32437](https://github.com/golang/go/issues/32437) adds new syntax to the language to make the `if err != nil { return ... }` boilerplate less cumbersome.
There are various alternative proposals: [#32804](https://github.com/golang/go/issues/32804) and [#32811](https://github.com/golang/go/issues/32811) as the original one is not universally loved.
To throw another alternative in the mix: Why not keep it as is? 
I've come to like the explicit nature of the if `err != nil` construct and as such I don't understand why we need new syntax for this. Is it really that bad?

  [miekg](https://github.com/miekg) commented on Jun 28, 2019

Honestly I'm with miekg here. I dont think it's that bad. Also if you dont want to type you can simply setup a simple remap for this like [@ThePrimeagen](https://youtu.be/lvKQh3Od6V4?si=ZtSs9VW28FoAR65h&t=713) did.
so now you dont have to type this everytime.

`if err != nil { return ... }` is something you probably type if you write go, I dont think its a downside of the language. It just gets the job done.
isn't that the whole point of [Go-lang](https://news.ycombinator.com/item?id=13432199a) ? 


## References

- https://rauljordan.com/why-go-error-handling-is-awesome/
- https://dave.cheney.net/2020/02/23/the-zen-of-go
- https://go.dev/blog/error-handling-and-go
- https://github.com/golang/go/issues/32825
- https://jesseduffield.com/Gos-Shortcomings-1/
- https://news.ycombinator.com/item?id=13432199
