# weatherCLI
Elastio Rust Test Task Solution

## _Structure_

To keep things simple - complex methods to store information are not used:

Program uses 4 files to store and configure avalible weather apis

- apiNames -- names of avalible apis
- keys     -- keys for each api for 
- current  -- current id of an api
- format   -- format of the request for each api

This was implemented using placeholders for the city and key. Example for OpenWeather:

```
https://api.openweathermap.org/data/2.5/weather?q={C}&appid={K}
```

Where

{C} - city placeholder

{K} - key placeholder

During runtime selected city and api (it's key) is put instead of placeholders

## _How to use_

```
weatherCLI help
```
for list of available commands

<img src="readMe\1.png"></img>

```
weatherCLI list
```
gives list of available weather Api

<img src="readMe\2.png"></img>

```
weatherCLI get <cityname> 
```
to request api for information about the weather in the city <cityname>

```
weatherCLI configure <apiName>
```
to change current weather api. If api was changed - 
 
 
Example of getting info from two seperate sources

<img src="readMe\4.png"></img>
  
