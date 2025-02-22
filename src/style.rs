use yew::{html, Html};

pub fn get_page_style() -> Html {
    html!(
        <style>
            { r#"
    body {
      font-family: Arial, sans-serif;
      margin: 20px;
    }

    .cv-container {
        width: 794px;
        margin: 0 auto;
        padding: 20px;
        background-color: white;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    }
    
    h1,
    h2,
    h3,
    h4 {
      color: #2A92C8;
    }

    .separate-line {
      height: 1px; 
      margin: 0px 30px;
      background-color: #2A92C8;
    }

    .fullname {
      margin-bottom: 0px;
    }

    .job-title {
      font-size: 20px;
      margin-top: 0px;
    }

    .icon-info {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 14px;
      margin: 6px 0px;
    }

    .clickable-link {
      color: #CDD6F4;
      text-decoration: none;
    }

    .clickable-link:hover {
      color: #89B4FA;
      text-decoration: underline;
    }

    .ref-link:has(:only-child) .icon-info {
      justify-content: flex-start;
    }
    
    .contact-info {
      margin-bottom: 20px;
    }

    .summary,
    .education,
    .projects,
    .skills,
    .languages,
    .interests {
      text-align: center;
      margin-bottom: 20px;
    }
    
    ul {
      list-style-type: disc;
      margin-left: 20px;
    }
    
    .section-title {
      font-weight: bold;
    }
            "# }
        </style>
    )
}
