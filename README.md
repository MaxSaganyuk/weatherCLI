# weatherCLI
Elastio Rust Test Task Solution

## _Structure_

Program uses 4 files to store and configure avalible weather apis

- apiNames -- names of avalible apis
- format   -- format of the request for each api
This was implemented using placeholders for the city and key. Example for OpenWeather:

https://api.openweathermap.org/data/2.5/weather?q={C}&appid={K}

Where

{C} - city placeholder

{K} - key placeholder

During runtime selected city and api (it's key) is put instead of placeholders



- keys     -- keys for each api for 
- current  -- current id of an api

## _How to use_

Type _help_ for list of available commands



Type _list_ for list of available weather Api


Type _configure_ <apiName> to change current weather api. If api was changed - 
  
Id of the current weather
