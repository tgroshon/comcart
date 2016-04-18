Comcart
=======

Parse a common cartridge archive and return a summary of the contents.

*Note: Modeled after the Instructure implementation of Common Cartridges*

Returns a summary struct of following data:

 - general
   - title
   - description
   - keyword
   - copyright
 - modules
   - title
   - items

Coming soon:

 - assignments
 - assessments (quizzes)
 - discussion topics
 - pages (web content)
 - outcomes
 - web links

## Development ##

TODO:

 - Read XML files for all resources

Resource type patterns:

 - assignment: `/assignment|associatedcontent\/imscc_xmlv1p1\/learning\-application\-resource/`
 - assessment: `/assessment|quiz/`
 - discussion: `/imsdt/`
 - page: `/webcontent/`
 - web link: `/wl/`

