# weatherCLI
Elastio Rust Test Task Solution

Was created and tested on Linux (Lubuntu) via virtual machine

## _Structure_

Two weather api implemented - OpenWeather and weatherapi

To keep things simple - complex methods to store information are not used:

Program uses 3 files to store and configure avalible weather apis

- apiNames -- names of avalible apis
- current  -- current id of an api (Id of an api corresponds to position in apiNames file)
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

 ### _Optiization for hypothetical future improvments_
 
 
Let's assume that this project will require more functional apis. The process to achieve this will require

- adding it's name to apiNames file
- adding it's request format to format file
- adding new code for parsing json for this new api in this section of the code to connect_to_api function

<img src="readMe\7.png"></img>

Process is automated enough to not require any additional work

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


 If user tries to receive infomation from provider without configuring his key first - user will get an error

<img src="readMe\4.png"></img>


 Configuring/setting the key looks like this


```
weatherCLI configure <apiName> <key>
```

<img src="readMe\3.png"></img>

After configuration - user can recive information from providers

```
weatherCLI get <cityname> [api_name]
```

to request api for information about the weather in the city <cityname>. Using [api_name] api. By default parameter _api_name_ is using last used api - so user is not obligated to enter prefered api each time. (OpenWeather is used if no apis were ever used)

Demonstration:
 
<img src="readMe\6.png"></img>

- First call - information is requested without specifying the provider, information is received from default provider OpenWeather
- Second call - information is requested with specifing the provider weatherapi, information is received from new provider weatherapi and "current" file changes it's value to 1
- Third call - information is requested without specifying the provider, information is received from last used provider, in this case it's weatherapi
- Fourth call - information is requested with specifing the provider weatherapi, information is received from new provider OpenWeather and "current" file changes it's value to 0
  
