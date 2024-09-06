Libs in Rust as known as crates.


## Planning & Organisation

1. __Obsidian__
2. ~~__Trello__~~ __Github Projects__
3. __Draw SQL__
4. __CloudCraft__ (Interfaces with AWS to build infrastructure diagrams)
___

## Langs & Libs

#### Python (Dashboard)
1. __Streamlit__ for dashboard
2. __Psycopg2__ to communicate with database

#### Rust (Backend)
1. __tokio__ to make a non-blocking app
2. __Reqwest__ to make the requests (consider Rocket)
3. __Tokio-postgres__ to communicate with psql
4. __SQLX__ to send SQL requests and validate them _before_ runtime
___

## Stretch Goals (Cloud & Remote)

1. AWS
2. Terraform
3. La Frite (Micro computer as host)

___

## Data Sources 
(For Minimum Viable Product)

### Data For Minimum Viable Product

- ['Official' UK inflation rates](https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/d7g7/mm23)

- Gold price (yFin API)
- BTC price (yFin API)
- [GDP](https://cy.ons.gov.uk/economy/grossdomesticproductgdp/datasets/gdpmonthlyestimateuktimeseriesdataset)(Direct file download link [here](https://cy.ons.gov.uk/file?uri=/economy/grossdomesticproductgdp/datasets/gdpmonthlyestimateuktimeseriesdataset/current/mgdp.csv))
- House price data ([Land registry SPARQL query](https://landregistry.data.gov.uk/app/qonsole))
- [Wages](https://www.ons.gov.uk/employmentandlabourmarket/peopleinwork/employmentandemployeetypes/bulletins/averageweeklyearningsingreatbritain/october2023)or [here](https://www.ons.gov.uk/employmentandlabourmarket/peopleinwork/earningsandworkinghours/bulletins/annualsurveyofhoursandearnings/2023)
- [National debt](https://www.statista.com/statistics/282647/government-debt-uk/) (as a [% of GDP](https://www.ons.gov.uk/economy/governmentpublicsectorandtaxes/publicspending/bulletins/ukgovernmentdebtanddeficitforeurostatmaast/september2020))
- [Petrol](https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/czmk/mm23)
- [Diesel](https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/czml/mm23)
- [Gas](https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/l53f/mm23) ([£](https://www.ofgem.gov.uk/all-available-charts?keyword=gas&sort=relevance) )
- [Electricity](https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/l53e/mm23) ([£](https://www.ofgem.gov.uk/all-available-charts?keyword=electricity&sort=relevance))
- FTSE 100 (yFin API)
- FTSE 250 (yFin API)

Maybe get the gold price from 1971 smooth it a lot, and then use the 1971 price a benchmark for real value and any increase in price is inflation (assumes Gold hans't changed in value in real terms). As well as using CPI.
https://www.chards.co.uk/gold-price/gold-price-history

 \* For ONS links you can sometimes just put /data at the end of the URL to make an API request. 

\* To get data from Ofgem website will need to use **fantoccini** to automate browser clicking and data download. 

\* Will have to scrape it from Statisa. Go to inspect element, and search '534.2' (debt in 2000/2001) and it will show you the list of figures for each year in the js/css.
