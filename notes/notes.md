# How this will work

A source file [`\artifacts\resume.md`]("C:\projects\resume-builder\artifacts\resume.md") is the source of truth, and will have all the information about the person who's resume is being created. It uses headings and lists to keep content structured and readable.

Some sort of tags will be used to divide all information, 

This will then be converted into a `.docx` file and a `.pdf` file

Feature list:
- [ ] Variants - different projects and skillsets tagged for different types of jobs
- [ ] Use [egui](https://github.com/emilk/egui)
- [ ] Import to source file from `.docx` or `.pdf`

Future
- [ ] Parse job description for key requirements
- [ ] Evidence and gap report
- [ ] Suggested resume edits
- [ ] Cover letter assistance
