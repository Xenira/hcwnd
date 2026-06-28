use maud::{html, Markup};

const NAME: &str = "HCWND";
const LAST_UPDATED: &str = "2026-03-22";
const WEBSITE: &str = "https://hardcore-will-never.diy";
const CONTACT_EMAIL: &str = "privacy@hardcore-will-never.diy";

pub fn data_protection() -> Markup {
    html! {
        h1 { "Privacy Policy for " (NAME) }
        strong { "Last updated: " } ( LAST_UPDATED )

        h2 { "Introduction" }
        p { (NAME) "('we, 'our', or 'us') operates " (WEBSITE) " (the 'Service'). This page informs you of our policies regarding the collection, use, and disclosure of personal data when you use our Service and the choices you have associated with that data." }
        h2 { "Information We Collect" }

        h3 { "Personal Information" }
        p { "We may collect personally identifiable information that you provide to us, such as:" }
        ul {
            li { "Email addresses" }
            li { "Account information and preferences" }
            li { "Your username will be attached to any content you create on the platform, such as events and edits." }
            li { "Moderation actions taken against your account will be logged and may be visible to other users." }
        }
        h3 { "Cookies and Tracking Data" }
        p { "We use cookies and similar tracking technologies to track activity on our Service and store certain information. You can instruct your browser to refuse all cookies or to indicate when a cookie is being sent." }

        h2 { "How We Use Your Information" }
        p { "We use the collected data for various purposes:" }
        ul {
            li { "To provide and maintain our Service" }
            li { "To notify you about changes to our Service" }
            li { "To monitor the usage of our Service" }
            li { "To detect, prevent and address technical issues" }
        }

        h2 { "Data Sharing and Disclosure" }
        p { "We do not share your personal data with third parties except as described in this policy." }
        strong { "We do not sell your personal data to third parties." }

        h2 { "Data Retention" }
        p { "We retain your personal information only for as long as necessary for the purposes set out in this Privacy Policy. We will retain and use your information to comply with our legal obligations, resolve disputes, and enforce our policies." }

        h2 { "Your Data Rights" }
        p { "You have certain rights regarding your personal data:" }
        ul {
            li { strong { "Right to Delete" } "You can request deletion of your personal data" }
            li { strong { "Right to Access" } "You can request copies of your personal data" }
            li { strong { "Right to Rectification" } "You can request correction of inaccurate data" }
            li { strong { "Right to Object" } "You can object to our processing of your data" }
        }
        p { "To exercise these rights, please contact us at" (CONTACT_EMAIL) "." }

        h2 { "GDPR Compliance" }
        p { "If you are a resident of the European Economic Area (EEA), you have certain data protection rights. We aim to take reasonable steps to allow you to correct, amend, delete, or limit the use of your personal data. " }

        h2 { "CCPA Compliance" }
        p { "If you are a California resident, you have certain rights under the California Consumer Privacy Act (CCPA), including the right to know what personal information we collect, the right to delete personal information, and the right to opt-out of the sale of personal information." }

        h2 { "Security" }
        p { "The security of your data is important to us. We implement appropriate technical and organizational security measures, including encryption, to protect your personal information." }

        h2 { "Children's Privacy" }
        p { "Our Service is not intended for use by children under the age of 18. We do not knowingly collect personally identifiable information from children under 18. If you are a parent or guardian and you are aware that your child has provided us with personal data, please contact us. " }

        h2 { "Changes to This Privacy Policy" }
        p { "We may update our Privacy Policy from time to time. We will notify you of any changes by posting the new Privacy Policy on this page and updating the 'Last updated' date." }

        h2 { "Contact Information" }
        p { "If you have any questions about this Privacy Policy, please contact us:" }
        ul {
            li { "By email: " (CONTACT_EMAIL) }
            li { "Through our website: " (WEBSITE) }
        }
    }
}
