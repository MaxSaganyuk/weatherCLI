# weatherCLI
Elastio Rust Test Task Solution

## _Structure_

Two weather api implemented - OpenWeather and weatherapi

To keep things simple - complex methods to store information are not used:

Program uses 3 files to store and configure avalible weather apis

- apiNames -- names of avalible apis
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

For key storage - 2 files named accordingly to api names are created. 

To confirm a key user must use _configure_ command

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
weatherCLI get <cityname> [api_name]
```
to request api for information about the weather in the city <cityname>. Using [api_name] api. By default parameter _api_name_ is using last used api - so user is not obligated to enter prefered api each time. (OpenWeather is used if no apis were ever used)

 If user tries to receive infomation from provider without configuring his key first - user will get an error

 
 Configuring/setting the key looks like this

```
weatherCLI configure <apiName> <key>
```
 

  
