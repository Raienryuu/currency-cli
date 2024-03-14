# Currency-converter-cli

Convert currencies through cli using public api to keep exchange rates up to date.


cargo run AMOUNT FROM_CURRENCY TO_CURRENCY

Usage example: </br>
<code>
cargo run 15 PLN EUR
</code>

Example output: </br>
![img.png](img.png)

-Supports caching data in local db through ruslite
