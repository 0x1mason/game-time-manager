# Game Time Manager, the Game Timer for Grown Ups

I'm excited to announce Game Time Manager (GTM), a free, open-source tool for managing and measuring your play time on Windows PC. GTM is a tool developed by adult gamers, for adult gamers. Now in early development, its primary feature is periodically showing an overlay that informs you how long you've been playing. Future features will include historical metrics and a GUI. The goal is to help you balance your gaming with the rest of your life.

I created Game Time Manager for a few reasons:

 - Tracking time while you play is really hard (more on that below).
 - >95% of PC gamers are over the age of 18, with 35 being the average age. However, many time monitoring apps are for parents who want to police their kids.
 - A lot of people manually track and manage their time when they're using a machine that can do it for them.
 - I want to know what I'm putting on my computer. GTM is open source, which means that everyone can see exactly what they're installing.

## Time flies when you're having fun

Our perception of time changes when we game. If you've ever thought you've been playing for an hour, only to discover that it's been two hours, then you know exactly what I mean. Scientists call this phenomenon "time compression" and it is a by product how our brains work. While time compression isn't unique to video gaming, it's definitely a major part of the gaming experience. In fact, time distortion is typically seen as a hallmark of a good game.

Now consider the fact that most PC gamers are adults. Most adults have responsibilities. Most responsibilities require keeping track of time. Yet when we play video games, our brains lose track of time. This 
 
## How it works

Game Time Manager starts along with your computer. Every few seconds, it checks if Windows is allowing notifications, those annoying messages from the System Tray. Because games generally suppress notifications, notification status is a good indication that a game is running. If notifications are suppressed and the window in focus occupies the entire screen, GTM assumes it's a game. It's an imprecise heuristic to be sure, but it works quite well in my experience. If you find that GTM is making bad assumptions, you can add applications to the ignore list.

Once Game Time Manager thinks a game is running, it periodically displays a small, non-disruptive overlay that indicates how long the game has been running. You can configure where and how long it displays. In general, you shouldn't notice any impact on your machine. Written in a modern, fast, and secure programming language, GTM typically uses less than 10MB of memory and its CPU usage is almost indiscernible. It also runs in its own process, so you don't need to worry about it interfering with your game or mods.

It works with most games, except with those that only run in Fullscreen Exclusive mode. That shouldn't be a problem for most newer games. If you aren't seeing the overlay, check the game's video options and see if you can change to Borderless Windowed mode. Note that what many games call "Fullscreen" is actually Borderless Windowed.

