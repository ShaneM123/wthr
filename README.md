<h1><code>wthr</code></h1>
<p>A Cli app designed to give you the weather of any location</p>
<p>work in progress, currently working on hosting the backend. Want it to look something like this:</p>
<p>$: wthr Berlin DE today </p>
<p>$: Sunny, 25 Degrees Celsius  </p>

<h2> to run </h2>
<p> in its current prototype state it requires rust installed and an api key for openweather.org (this will be fixed shortly)</p>

<p> firstly, please setup the backend here. </p>
<p>once the backend is up and running type the following in the command line:</p>
<code>cargo run 2964180
</code>
<p></p>
<p> <code>2964180</code> is the city id and can be replaced.</p>

<p>valid city id's can be found in the citylist.json file and api keys can be obtained by signing up to openweather.org</p>