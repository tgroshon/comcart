mod handler;
mod builder;
mod index_tracker;

use common::{ Manifest };
use std::io::{ Read };
use summarize::manifest::handler::ManifestHandler;
use summarize::utils::handle_parse;

pub fn parse<R: Read>(manifest: R) -> Manifest {
    let mut handler = ManifestHandler::new();
    handle_parse(manifest, &mut handler);
    let manifest = handler.finalize_manifest();
    println!("{:?}", &manifest.general);
    manifest
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_general() {
        let manifest = parse(xml_string().as_bytes());
        assert_eq!(manifest.general.title, "A Course");
        assert_eq!(manifest.general.description, "test this course");
        assert_eq!(manifest.general.copyright, "Private");
    }

    #[test]
    fn parses_modules() {
        let manifest = parse(xml_string().as_bytes());
        assert_eq!(manifest.modules.len(), 2);
        assert!(manifest.modules.iter().any(|module| module.title == "Mod Testing" || module.title == "Module 1"));
    }

    #[test]
    fn parses_resources() {
        let manifest = parse(xml_string().as_bytes());
        assert_eq!(manifest.resources.len(), 9);
    }

    fn xml_string<'a>() -> &'a str {
        r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <manifest identifier="i3ebf4e9fe673c98a2e10715ec293b6bf" xmlns="http://www.imsglobal.org/xsd/imsccv1p1/imscp_v1p1" xmlns:lom="http://ltsc.ieee.org/xsd/imsccv1p1/LOM/resource" xmlns:lomimscc="http://ltsc.ieee.org/xsd/imsccv1p1/LOM/manifest">
          <metadata>
            <schema>IMS Common Cartridge</schema>
            <schemaversion>1.1.0</schemaversion>
            <lomimscc:lom>
              <lomimscc:general>
                <lomimscc:title>
                  <lomimscc:string>A Course</lomimscc:string>
                </lomimscc:title>
                <lomimscc:description>
                  <lomimscc:string>test this course</lomimscc:string>
                </lomimscc:description>
              </lomimscc:general>
              <lomimscc:lifeCycle>
                <lomimscc:contribute>
                  <lomimscc:date>
                    <lomimscc:dateTime>2016-03-15</lomimscc:dateTime>
                  </lomimscc:date>
                </lomimscc:contribute>
              </lomimscc:lifeCycle>
              <lomimscc:rights>
                <lomimscc:copyrightAndOtherRestrictions>
                  <lomimscc:value>yes</lomimscc:value>
                </lomimscc:copyrightAndOtherRestrictions>
                <lomimscc:description>
                  <lomimscc:string>Private</lomimscc:string>
                </lomimscc:description>
              </lomimscc:rights>
            </lomimscc:lom>
          </metadata>
          <organizations>
            <organization identifier="org_1" structure="rooted-hierarchy">
              <item identifier="LearningModules">
                <item identifier="id7d0efe702450aa5e9e1664dfeaa94ee">
                  <title>Mod Testing</title>
                  <item identifier="ib3ace830fa129c9d1c24702eebf3ebb4" identifierref="ib81691c293cfcd8474622cfd8c0047f5">
                    <title>mod debug</title>
                  </item>
                </item>
                <item identifier="i1ca3ec3f790e6285369f267c1be1a022">
                  <title>Module 1</title>
                  <item identifier="i6760f3c4e14f35246d3c1b0cdab71787" identifierref="i86ed42b01897fba5a7126d18558fd7a8">
                    <title>Assignment 1</title>
                  </item>
                  <item identifier="i922878463b7905cdaec01dc3883cc08d" identifierref="i5ae2ebabbdca02262357c21db54aec9b">
                    <title>Quiz 1</title>
                  </item>
                </item>
              </item>
            </organization>
          </organizations>
          <resources>
            <resource identifier="i5e0d8279664539be677db96c71643966_syllabus" type="associatedcontent/imscc_xmlv1p1/learning-application-resource" href="course_settings/syllabus.html" intendeduse="syllabus">
              <file href="course_settings/syllabus.html"/>
            </resource>
            <resource identifier="i5e0d8279664539be677db96c71643966" type="associatedcontent/imscc_xmlv1p1/learning-application-resource" href="course_settings/canvas_export.txt">
              <file href="course_settings/course_settings.xml"/>
              <file href="course_settings/module_meta.xml"/>
              <file href="course_settings/assignment_groups.xml"/>
              <file href="course_settings/grading_standards.xml"/>
              <file href="course_settings/rubrics.xml"/>
              <file href="course_settings/learning_outcomes.xml"/>
              <file href="course_settings/files_meta.xml"/>
              <file href="course_settings/events.xml"/>
              <file href="course_settings/media_tracks.xml"/>
              <file href="course_settings/canvas_export.txt"/>
            </resource>
            <resource identifier="ia88f49c25e1b684e311b2ff3da9c3780" type="webcontent" href="wiki_content/front-page.html">
              <file href="wiki_content/front-page.html"/>
            </resource>
            <resource identifier="iadc3bb46492a88f5dec93121538151df" type="webcontent" href="wiki_content/page-hidden-from-students.html">
              <file href="wiki_content/page-hidden-from-students.html"/>
            </resource>
            <resource identifier="i86ed42b01897fba5a7126d18558fd7a8" type="associatedcontent/imscc_xmlv1p1/learning-application-resource" href="i86ed42b01897fba5a7126d18558fd7a8/assignment-1.html">
              <file href="i86ed42b01897fba5a7126d18558fd7a8/assignment-1.html"/>
              <file href="i86ed42b01897fba5a7126d18558fd7a8/assignment_settings.xml"/>
            </resource>
            <resource identifier="id05f7ff3bfefb887478eed302e9e7577" type="imsdt_xmlv1p1">
              <file href="id05f7ff3bfefb887478eed302e9e7577.xml"/>
              <dependency identifierref="id7e712a7065dd41d746846c5898bd9d3"/>
            </resource>
            <resource identifier="id7e712a7065dd41d746846c5898bd9d3" type="associatedcontent/imscc_xmlv1p1/learning-application-resource" href="id7e712a7065dd41d746846c5898bd9d3.xml">
              <file href="id7e712a7065dd41d746846c5898bd9d3.xml"/>
            </resource>
            <resource identifier="i5ae2ebabbdca02262357c21db54aec9b" type="imsqti_xmlv1p2/imscc_xmlv1p1/assessment">
              <file href="i5ae2ebabbdca02262357c21db54aec9b/assessment_qti.xml"/>
              <dependency identifierref="iac561ca66d48400a9f8fd25c86454d4b"/>
            </resource>
            <resource identifier="iac561ca66d48400a9f8fd25c86454d4b" type="associatedcontent/imscc_xmlv1p1/learning-application-resource" href="i5ae2ebabbdca02262357c21db54aec9b/assessment_meta.xml">
              <file href="i5ae2ebabbdca02262357c21db54aec9b/assessment_meta.xml"/>
              <file href="non_cc_assessments/i5ae2ebabbdca02262357c21db54aec9b.xml.qti"/>
            </resource>
          </resources>
        </manifest>
      "#
    }


}
