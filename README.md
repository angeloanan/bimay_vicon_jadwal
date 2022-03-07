# BINUSMaya Schedules

📅 | Shows BINUSMaya schedule on your terminal (and auto opens upcoming video conference class!) 

---

![Image showing the app working](./.github/resources/demo.png)

## Motivation

I found it hassling to basically get my schedule off of BINUS' myclass website. Simply getting the Video Conference meeting link everytime there is class is really painful.

The steps that I would take to get my schedule is:

1. Open browser
2. Type `bm5` and let Autocomplete completes to `https://bm5jadwal.azurewebsites.net/`
3. Type `CTRL + SHIFT + X` to unlock my password manager and type my password
4. Choose my credentials and click login
5. Wait for a bit and click link that is the highest entry on the table

Mind you, most of the time, I am half awake at this state. I would have typo'd my password alot and simply navigating to it is painful.

With this simple CLI app, I hope that I can incrementally "replace" the schedule website such that its much friendlier and easier to use (if not for ya'll, atleast for me). I will add more features that will make life easier.

## Implemented Features

* Getting today's schedule, sorted by time and filtered by already happenning
* Auto opening upcoming / current class video conference link