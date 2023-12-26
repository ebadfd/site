---
title: CTF writeup for BOOTCAMP CTF challenge
date: "2021-08-05"
tags:
  - beginner
  - ctf
---

Hello, you amazing hackers! Welcome back to another CTF walkthrough. so this time we are going to go through the Bootcamp CTF conducted by owasp community from FOSS NSBM. so this was really beginner-friendly and easy CTF. also I want to mention you can find the challengers in a GitHub repository after the CTF is over so that you can try to play this challenger yourself. so without further due let's get started with hacking. also, you can find the live walkthrough of this Bootcamp below.

<yt-video id="zmBRDeUpaDo"> </yt-video>

![](/img/screely-1624134704670-1024x573.png)

So the image above was the dashboard for ctf challengers. so we will start from the first one

## can you find the sup3r S3cr3t key ?

can you find the sup3r S3cr3t key? is the first challenge in the Bootcamp CTF. so in the challenge description you can see "find the flag, submit" it doesn't give us much information right? so let's view the hint so in the hint we can see REVERSE? keyword. ahh, interesting. and for the task files, we can see it gives us a binary file and when we run it

![](/img/carbon56-1024x354.png)

so it seams like we have to enter a key to get the flag, but how can we find the key? so for this I thought of using IDA Freeware (Interactive Disassembler) this is basically a disassembler also you can use Ghidra too.

![](/img/screely-1624135264896-1024x598.png)

oh wow! here immediately we can see the secret token. great! so I think now we can just use this token to get the flag! great also if you view the hex you can see the flag too I have shown it in the below diagram

![](/img/carbon57-1-1024x403.png)

also we can use that secret key to get the flag... as shown in the diagram below

![](/img/carbon58-2-1024x258.png)

and in case you need a show and very simple way to do it you can just cat the file or view the strings.

![](/img/carbon59-1024x500.png)

#### Beautiful Mountain

So the second challenge was Beautiful Mountain , in this challenge description we cant see anything really interesting all we can see is just "hmm..." this wont help us right ? so let's go ahed and view what are in the task files.

![](/img/beautiful-1024x680.jpg)

and yes! the task file is basically this. is this a typical image? I don't think so. let's go into our terminal and try stenography to see if something hidden inside this image.

this this case im using steghide, you can also use the other tools syntax I used is mentioned below.

```bash
steghide --extract -sf beautiful.jpg
```

![](/img/carbon60-1-1024x581.png)

and yes! it did work we successfully got the flag. good job. so lets move on to the next challenge

#### Something is wrong with my image

in the description of this challenge we can see something saying "**Can you fix this for me ?**" but in the task files we can see an image. seems interesting huh? Let's first get the image and see what's going on that

![](/img/2021-06-20-02_29_30-broken.jpg-JPEG-Image-—-Firefox-Developer-Edition-Private-Browsing-1024x123.png)

so you can see it says broken.jpg cant display because it contains errors. in this case you can use a tool like wget to get download this image.

```bash
wget https://downloads.hack.fossnsbm.org/challengers/broken.jpg --no-check-certificate
```

so after getting the image you can still see you can't open the image but why? so it seems like the image is corrupted but how can we know what is that? we'll for this type of case we can use tool like hex editor, so if you don't know hex editor is

<blockquote>A **hex editor** (or binary file **editor** or byte **editor**) is a computer program that allows for manipulation of the fundamental binary data that constitutes a computer file. The name '**hex**' comes from '**hexadecimal**': a standard numerical format for representing binary data.
> 
> </blockquote>

so if we view the binaries of this image we can see why it says this file can't be displayed.

    <code class="">hexeditor broken.jpg</code>

![](/img/2021-06-20-02_33_41-Kali-Linux-2021.1-vbox-amd64-Clone-Running-Oracle-VM-VirtualBox.png)

so as you can see in the above image the first values are changed to zero but in jpg file it should be

    <code class=""> FF D8 FF E0  00 10 4A 46   49 46 00 01 01 01 00 48 </code>

it seams like first two parts are changed. great! so lets manually try to change these values and see if we can at least view this image

![](/img/2021-06-20-02_41_05-Kali-Linux-2021.1-vbox-amd64-Clone-Running-Oracle-VM-VirtualBox-1024x392.png)

and yes! after changing the values we can see the image is actually is displayed! great but where is the flag? just like before let's try steghide and see if we can get the flag

![](/img/carbon61-1-1024x391.png)

and yes! we found another flag, and let's move on to the next challenge. a quick reminder here tho in this writeup I'm on going to walkthrough about the challengers in the CTF. so here I'm not going to show about the flags on the website. so let's skip the **what do you think about our cool website** challenge and then move on to the next challenge.

#### WHAT DO YOU THINK ABOUT MY MUSIC SKILLS?

so let's talk about this challenge now. in the challenge description, we can see it says "here's a song I played" and great! and for the task files, we can download the music.wav file. and for the hint, we can see something very very interesting. the hint basically says it all **"THIS IS DEEP"** so if you have some experience with steg challengers you probably have heard about **the **DeepSound tool. a deep sound is a tool that helps us to hide something inside an audio file. great everything seems clear. first, let's download the task files and see.

so you can see the audio file above, in this file, I didn't hear any weird breaking or courpted scenes. mostly if someone hides or something inside the audio files that happens. but in this case, we can't see anything like that ( if you are curious you can use tools like **sonic visualizer** to further enumeration ). since the hint itself mentioned about DeepSound let's try to use it first and see.

![](/img/2021-06-20-02_53_17-DeepSound-2.0.png)

haha great! just after you open the file ( use Open Carrier files tab to open a file ) you can see the file we are looking for xd , we got the flag.txt and you can use the "Extract secret files" tab to get the flag to you!

#### Can you decode this to me?

<blockquote>i encoded a file but i can't remember how to decode it can you help me?
> 
> </blockquote>

So for the challenge description we can see it says above , great . seams like we have to decode this to get the flag. cool so let's first download the task files and see what we have to do.

![](/img/2021-06-20-03_03_54-Kali-Linux-2021.1-vbox-amd64-Clone-Running-Oracle-VM-VirtualBox.png)

but wait the zip file is password protected! crap. the challenge doesn't say anything about password hmm. Let's try to crack the password first and see

first, let me explain how to crack a zip file. so for cracking the zip file I'm going to use John the Ripper (this is a free password cracking software) also if you are using apt package manager you can install this by simple** apt get install john **command.

so for cracking the password, we are using zip2john so basically to crack the zip file first we need to convert it to a format that the john the ripper tool can understand. and after that, we can crack it using a brute-force attack or dictionary attack if you are using brute force attack/password attack I can recommend you the rock you password list, you can download it from [here ](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwin5NDn1KTxAhWBgtgFHQeXDi0QFjAAegQIBRAD&url=https%3A%2F%2Fgithub.com%2Fbrannondorsey%2Fnaive-hashcat%2Freleases%2Fdownload%2Fdata%2Frockyou.txt&usg=AOvVaw3snAERl1mU6Ccr4WFEazBd)

the method is actually pretty straight forward i will leave the commands below

```bash
zip2john bubble.zip > bubble
john --wordlist=wordlist.txt bubble
```

![](/img/carbon62-1-1024x402.png)

great so now we have the unzipped file and we can see the encrypted file

```bash
'xivog-voluh-pukag-sahah-doboh-baleh-faleh-fafaf-cohyf-disef-disoh-zubah-zokeg-noveh-cisog-pikyh-gafyh-zifoh-zuhif-bucyf-notyg-hakah-kogah-dizix'
```

so if we view the challenge hint we can see a hint, saying bubble.. this is weird right? but actually, it's not it gives us what to do. is shows us the direction so if you don't know in python we have a library called bubblepy, seems like this is the way. let's give this a go

so in-case you are interested you can find the documentation for bubblepy below

[https://pypi.org/project/bubblepy/](https://pypi.org/project/bubblepy/)

so to install this library you can use

```bash
pip install bubblepy
```

```python
 from bubblepy import BubbleBabble
 bb = BubbleBabble()
 bb.decode('{encode value here}')
```

![](/img/carbon63-1-1024x417.png)

and just like that ! we successfully finished this challenge! great... and now let's move on to the next one.

#### I just learned python!!

<blockquote>here's a dumb code i wrote to print whatever u enter back
> 
> </blockquote>

so for the description of this challenge, we can see this, but if we view the hint we can see it says, or is it really dumb.. interesting right? let's get the task files and see what we can do here.

so for the task files, we download a pyc file, so if you don't know what is a pyc file **pyc files** are created by the Python interpreter when a . py **file** is imported. so first we'll run this file then we will see what we can get

![](/img/carbon64-1-1024x253.png)

so as the description says, this actually seems very dumb. but is it really? to figure this out we have two options. we can FUZZ the input and see if we are getting something else. or else we can even try to decompile pyc file and see what does the code do so first let's try the second method and see

so for decompile the file you can use a tool like uncompile6 this is actually a python library so you can just install this with pip

[https://github.com/rocky/python-uncompyle6/](https://github.com/rocky/python-uncompyle6/)

you can find more information about this tool from the above link so here I'm going to use it to see what is the decompiled output

![](/img/carbon65-1-1024x372.png)

haha great! seams like it gave us the python code,

![](/img/carbon66-1-1024x238.png)

and as above you can see the code and the flag. haha great! good job if you completed this. lets move on to the next challenge.

#### API

in the hint we can see it says can you find the key. haha great! also it gives us the challenge website. which is https://api.hack.fossnsbm.org/ and when we go there we can see it asks for the key "please enter the key (P)" ( since this is a API we need to send it via request method ) so how can we find the key ? first let's try some dir fuzzing. here I'm going to use gobuster. since its easy and does the job done real quick.

so first im gonna run gobuster scan and see if we can find something

```bash
gobuster -u https://api.hack.fossnsbm.org/ -w wordlist.txt -x bak -k
```

here I'm using **-x** for mention that I'm looking for bak files , and I'm using **-K** flag to disable certificate checks.

![](/img/carbon67-1-1024x432.png)

and great! we got **status 200 for key.bak **, that means there is a file called key.bak. great so let's first get that file and see whats in there

![](/img/carbon68-1024x223.png)

and we can see the key.bak file now. great! so whats next let's request this to the website to see if we can get the flag.

```
84107418413276471232732487324602
```

[https://api.hack.fossnsbm.org/?p=84107418413276471232732487324602](https://api.hack.fossnsbm.org/?p=84107418413276471232732487324602)

![](/img/2021-06-21-23_39_17-Firefox-Developer-Edition.png)

hmm and after the request we can see it says what are the other methords you have.. so if you dont know here we are performing a get request. so let's try to do a post request and see what's going to happen.

so for make things simple i wrote a very simple html code with a flag that performs the action to this website ( post ) so you can see it below

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Document</title>
  </head>
  <body>
    <form action="https://api.hack.fossnsbm.org/index.php" method="post">
      <input type="name" name="p" />
      <input type="submit" value="click" />
    </form>
  </body>
</html>
```

![](/img/carbon70-1-1024x387.png)

so as you can see it's very simple. so if you don't understand it basically performs an action to the given URL with a post request and it sends the parameter P with the value we enter into the form. so let's see what happens if we enter our key here.

so as you can see in the above video snippet we successfully got the flag. also, there are a lot of other ways we can do it. but I thought doing this will be more helpful for beginners and everyone.

#### Awsome SocialMedia

<blockquote>Make sure you check our social media
> 
> </blockquote>

so this challenge is a OSNIT challenge , it says make sure you check our social media so to find the flag here you can go the the facebook page of foss nsbm

[https://www.facebook.com/foss.nsbm](https://www.facebook.com/foss.nsbm)

and in the first post about the boot-camp you can find the flag.

![](/img/2021-06-22-01_52_38-1-FOSS-Community-NSBM-_-Facebook-—-Firefox-Developer-Edition.png)

---

## SQL CHALLENGE

![](/img/Screenshot-2021-06-21-at-23-54-02-BOOTCAMP-CTF-1024x530.png)

[https://sql.hack.fossnsbm.org/](https://sql.hack.fossnsbm.org/)

Now let's get start with the sql challenge. so first in this challenge we dont have much info all we have is the url to inject and nothing more. but the title says it all. so first when we go to the website we can see a login screen.

![](/img/screely-1624301666460-1024x558.png)

Great, so for the login screen I tried brute force the password. You can even try the SQL injection here but to make things more simple I thought of brute-forcing the login and see what's next. so the username and password were actually pretty guessable. it was

    <code class="">admin
    bootcampnsbm</code>

and when the login is success we get a redirect to welcome.php site

![](/img/2021-06-22-00_26_56-.png)

this is something like search office panel. great! so the search function really got my attention here. so what i did was i checked the request going on with the search function.

![](/img/2021-06-22-00_29_14-.png)

so here I used burp to catch the request, also here just want to mention before you guys get confused I'm going this on locally. not in the https://sql.hack.fossnsbm.org/ the request and everything are just the same so don't get confused. methods and everything the same.

so from the request we can see some interesting things. there is a parameter called search and it contain the our search term and also it sends a post request to welcome.php file to give us the output. great so what if we enter some invalid character as search , can we make some error on the SQL syntax going in the background ? lets test.

![](/img/got-error-1024x281.png)

ahh great! so we managed to make an error. so now we can confirm that this is vulnerable to SQL injection ( to perform the error I used **''** because this will break the SQL syntax )

### So how can we exploit this ?

to exploit this I thought of using [sqlmap](https://sqlmap.org/) so first to exploit this I saved the request to a request.txt file.

![](/img/2021-06-22-00_37_29-.png)

and it should save the output in XML format

![](/img/carbon72-1-1024x909.png)

great! so now we have the request and all, so what's next? now it's time to just wait and see how sqlmap will do the magic for us.

```bash
sqlmap -r request.txt --dbms=mysql --dump
```

![](/img/carbon74-1-914x1024.png)

so just wait for some time and you should get all the tables. also if this was not clear for you you can find a simple video of doing this. also I just wanted to mention that I will soon upload a detailed walkthrough of this SQL injection very soon! so stay tuned for that. I will update here when it's completed

<yt-video id="LUmP4e4G5qs"> </yt-video>

so after successfully exploiting the SQL injection. Let's move on to the next challenge

---

## XSS CHALLENGE

![](/img/Screenshot-2021-06-22-at-00-56-24-BOOTCAMP-CTF-1024x530.png)

great. we are in the final challenge now. let's see what we have to do here.

so when we go the [xss.hack.fossnsbm.org](https://xss.hack.fossnsbm.org/) we can see a simple note-taking type application. seems interesting so let's see what is does

![](/img/Screenshot-2021-06-22-at-01-21-43-Bootcamp-Hackathon-Notes-1024x530.png)

so as we can see it shows whatever we enter in the note section. cool! so in the above, I have tried by adding some HTML tags to see if it renders but as you can see in your note section it doesn't seem to work, seems like it ignores all the tags but let's check the source and see how it looks.

![](/img/2021-06-22-01_29_34-Bootcamp-Hackathon-_-Notes-—-Firefox-Developer-Edition.png)

foreget about rendering the html, here we can see something very interesting. so as you can see here the value we enter is inside of the value in input tag so what we can do now? lets try to close this using out injection and see what happens you can find the payload i used below.

```html
Hello ">
<h1>myinjection</h1>
```

![](/img/Screenshot-2021-06-22-at-01-31-41-Bootcamp-Hackathon-Notes-1024x530.png)

haha great! see what happened ? we successfully managed to inject some HTML code to the input tag. this is actually very common injection technique in bug bounty hunting. so far everything is going super smooth. so whats next ? let's injection some java-script as the same we did with the HTML

![](/img/2021-06-22-01_34_49-Bootcamp-Hackathon-_-Notes-—-Firefox-Developer-Edition.png)

and as you can see here we successfully trigerd the XSS injection. and after you click enter you should get a redirect to another page.

![](/img/Screenshot-2021-06-22-at-01-36-20-Bootcamp-Hackathon-Notes-1024x530.png)

in case you didn't notice what happened was after triggering the XSS injection you got assigned a cookie. and then you got a redirect to a welcome.php site. you can see javascript function that trigers this action below

![](/img/carbon75-1-1024x238.png)

so after you got redirected since the cookie you have is valid you will get redirected again to this hidden HTML page. and you can see the cookie that got assigned to you below.

![](/img/2021-06-22-01_39_21-Bootcamp-Hackathon-_-Notes-—-Firefox-Developer-Edition.png)

so far everything is going smooth but what do we have to do with this website ? why is it hidden ? haha just look at the source code and you will understand. in the source code you can find a flag.

`ZTA1Zk1FczRPRVJFTVY5VFgxUTJRa0ZXVkRSSlgwSmZXbEpOU2xrME4wbEtYMDFmVlZFMlFrOWFXVkJaT0gwPQ==`

you might think thats all. haha but there is one MORE....

![](/img/carbon76-1024x238.png)

if case you didnt notice the background image is called weird.jpg. what do you think ? haha yes its another stenography challenge.

![](/img/carbon77-1024x581.png)

---

and yes! finally, we got to an end! if you completed the challenge congrats! this was a very fun CTF and it was a pretty easy one. so if you are just starting out I think this was a great opportunity for you. in case you couldn't participate the challenge don't worry you can set up these challengers yourself by downloading all the task files from the Github repo. if you are having any problems with the challengers or if you need any help for the challengers feel free to contact me. you can send me an email at [dasithsv@gmail.com](mailto:dasithsv@gmail.com) I will help you with the challenges or setting up the challengers.

also shutout to the ctf winners

![](https://dev-sdfsdfsdfs.pantheonsite.io/wp-content/uploads/2021/07/651b9124-6aaa-4055-89e9-c4b8954d41fa-1024x1024.jpg)

so I really hope everyone enjoyed the CTF a lot! and we are hoping to come up with new more ctf's soon! until that stay safe and as always **keep On Hacking!**
