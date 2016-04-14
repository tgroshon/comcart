Comcart
=======

TODO:

 - Parse pretty type from long-form type on resources
 - Get "General" module specifics

Returns the following data:

 - general
   - title
   - description
   - keyword
   - copyright
 - outcomes
 - resources
   - assignments
   - assessments (quizzes)
   - discussion topics
   - pages (web content)
   - web links

Resource type patterns:

 - assignment: `/assignment|associatedcontent\/imscc_xmlv1p1\/learning\-application\-resource/`
 - assessment: `/assessment|quiz/`
 - discussion: `/imsdt/`
 - page: `/webcontent/`
 - web link: `/wl/`

