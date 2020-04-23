# Ncube Glossary

## Ncube

Based on a previsouly existing tool called Sugarcube, Ncube helps reasearchers to produce a set of verified data in order to make a statement about their area of research.

## Researcher

A researcher is any person that wants to use data as a tool to research and investigate their area of interest.

## Workspace

Any interaction with Ncube happens within a workspace. They resemble the idea of an individual project.

## Query

A query forms the source for one or many pieces of data. They tell processes where to look up up a particular piece of data in orderto fetch it. Examples for a query are a Youtube channel, a Tweet or a website URL.

## Data Unit

Individual pieces of data are called units of data. They represent a single thing that was fetched using a process. Sometimes a query maps to a single unit of data, but that's not necessarily always the case. When fetching a Tweet it forms a single unit of data, whereas a Youtube channel yields many units of data, one for each video that is part of the channel.

## Database

Every workspace has a single database. Any data that is fetched is stored in this one database.

## Segment

A segment is a narrow view into the database. It allows to codify repeated questions to the data of a workspace. There are two types of segments:

- A tagged segment is created based on one or more tags. Segments based on tags are used for structural organization. Membership of a unit of data in a tagged segment is un-ambigious.
- A search segment is created by applying a search query to the database. Results are ordered using a weight, the higher the weight the more likely it is an accurate result. Membership of a unit of data in a search segment therefore is ambigious and not guarenteed.

## Process

To fetch data for a query Ncube runs data processes based on Sugarcube. Each process fetches data from a different source or represents some other type of data process.

## Run Queue

Data processes have to run in the background. To manage those background tasks and to make sure not to overload the computer processes are queued up to run one after the other. This is the responsibility of the runqueue. When a process is started it is dispayched to the run queue to execute as soon as it is possible.

## Investigation

Based on a single database many types of investigations are possible, e.g. an investigation tries to answer a single question to the data or tries to make a statement about an event. One database can lead to many different investigations of different complexity and purpose.

## Verification Workflow

The basis of an investigation forms a process of verification. Each investigations has a single verification workflow. Individual units of data are added to the workflow, and are annotated and vetted in the process following the rules of the verification workflow. The goal of verification is to produce a set of data about which a statement can be made with confidence that it is accurate. Verification workflows can be constructed freely as is required by the needs of the investigation.

## Verification Stage

A verification workflow is broken down into several stages. Each stage can represent a milestone in the overall verification or an element of the organizational structure of the investigation. Each stage can require different sets of annotations. Each unit of data can reach the next verification stage once it passed the current stage. Pushing data like this from one stage to another makes sure that data experiences the proper due dilligence.

## Annotation

Every data that is fetched by a process cannot be modified. But researchers can enhance the data by annotating existing data with new information. It is clearly visible which data is original and which data are annotations. Annotations can be freely added as part of a verification workflow.

## Tag

Units of data can be tagged to mark them as being part of a category. The meaning of tags are up for the user but they can be used to sort and find related data.
