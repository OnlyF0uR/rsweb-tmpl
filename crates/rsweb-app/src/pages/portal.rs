use maud::{DOCTYPE, Markup, html};
use rsweb_auth::google_client_id;

use crate::components::load_theme::LOAD_THEME;

pub fn render() -> Markup {
    html! {
      (DOCTYPE)
      html {
        head {
          title { "Portal" }
          script defer src="/static/router.js" {}
          (LOAD_THEME)
          link data-dynamic rel="stylesheet" type="text/css" href="/static/app.css" {}
          style data-dynamic { r#"
                    #app {
                        display: flex;
                        height: 100vh;
                    }

                    .left {
                    	flex: 1;
		                position: relative;
                    }

                    .left img {
                        width: 100%;
                        height: 100%;
                        object-fit: cover;
                        display: block;
                    }

                    .shadow {
		                position: absolute;
		                top: 0;
		                left: 0;
		                width: 100%;
		                height: 100%;
		                background: linear-gradient(180deg, rgba(0, 0, 0, 0) 0%, rgba(0, 0, 0, 0.5) 100%);
	                }

                    .title {
                    	position: relative;
		                display: flex;
		                height: 100%;
		                flex-direction: column;
		                justify-content: center;
		                align-items: center;
                    }

                    .title h1 {
                    	font-size: 2.2rem;
                        color: white;
                    }

                    .title h2 {
                    	font-size: 1.2rem;
                        color: white;
                        font-weight: 500;
                    }

                    .right {
		                flex: 1;
		                background-color: white;
		                display: flex;
		                justify-content: center;
		                align-items: center;
	                }

	                .box {
		                max-width: 28rem;
		                width: 100%;
	                }

                    .username,
                    .email,
                    .password {
                        margin-bottom: 1rem;
                        display: flex;
                        flex-direction: column;
                        gap: 0.5rem;
                    }

                    .username {
                        display: none;
                    }

                    .field-label {
		                color: rgb(55 65 81);
		                font-weight: 500;
		                font-size: 1.125rem;
		                line-height: 1.75rem;
	                }
	                .field-input {
		                color: rgb(55 65 81);
		                font-size: 0.875rem;
		                line-height: 1.25rem;
		                padding-top: 0.75rem;
		                padding-bottom: 0.75rem;
		                padding-left: 1rem;
		                padding-right: 1rem;
		                border: 1px solid rgb(209 213 219);
		                border-radius: 0.5rem;
		                height: 2.5rem;
		                width: 100%;
	                }

                    .pwd-bar {
                    	position: relative;
		                width: 100%;
                    }

                    .pwd-bar button {
                        position: absolute;
			            right: 0;
			            top: 0;
			            bottom: 0;
			            display: flex;
			            align-items: center;
			            padding: 0 1rem;
			            background-color: transparent;
			            border: none;
			            cursor: pointer;
			            color: rgb(55 65 81);
			            font-size: 0.875rem;
			            line-height: 1.25rem;
                    }

                    .rem-for {
                    	display: flex;
		                justify-content: space-between;
		                align-items: center;
		                margin-bottom: 1rem;
		                color: rgb(75 85 99);
		                font-weight: 500;
		                font-size: 0.875rem;
		                line-height: 1.25rem;
                    }

                    .rem-for a {
                    	color: var(--theme);
			            font-weight: 500;
			            font-size: 0.875rem;
			            line-height: 1.25rem;
                    }

                    .rem-for a:hover {
                    	color: var(--theme-l);
                    }

                    .remember {
                        display: flex;
                        gap: 0.2rem;
                        align-items: center;
                    }

                    .continue {
                    	background-color: var(--theme);
		                color: white;
		                border: none;
		                border-radius: 0.5rem;
		                font-weight: 600;
		                font-size: 0.875rem;
		                line-height: 1.25rem;
		                padding-top: 0.75rem;
		                padding-bottom: 0.75rem;
		                padding-left: 1rem;
		                padding-right: 1rem;
		                display: flex;
		                justify-content: center;
		                align-items: center;
		                cursor: pointer;
		                height: 2.5rem;
		                width: 100%;
		                gap: 0.25rem;
		                margin-bottom: 1rem;
                    }

                    .sub-button-text {
                    	display: flex;
		                justify-content: space-around;
                        align-items: center;
		                color: rgb(75 85 99);
		                font-weight: 500;
		                font-size: 0.875rem;
		                line-height: 1.25rem;
                    }

                    .sub-button-text button {
                    	all: unset;
			            color: var(--theme);
			            cursor: pointer;
                        margin-left: 0.2rem;
                    }

                    .sub-button-text button:hover {
                    	color: var(--theme-l);
                    }

                    .g_id_signin {
                    	padding-bottom: 0.75rem;
                        display: flex;
                        justify-content: center;
                    }
                "# }
        }
        body {
          div id="app" {
            div class="left" {
              img src="/static/images/signin.jpg" alt="landing" {}
              div class="shadow" {
                div class="title" {
                  h1 { "Welcome Back" }
                  h2 { "We're so excited to see you again!" }
                }
              }
            }
            div class="right" {
              div class="box" {
                form action="/api/login" method="post" {
                  div class="username" {
                    label class="field-label" for="username" { "Username" }
                    input class="field-input" type="text" name="username" id="username" required placeholder="yourname" {}
                  }
                  div class="email" {
                    label class="field-label" for="email" { "Email" }
                    input class="field-input" type="email" name="email" id="email" required placeholder="you@example.com" {}
                  }
                  div class="password" {
                    label class="field-label" for="password" { "Password" }
                    div class="pwd-bar" {
                      input class="field-input" type="password" name="password" id="password" required placeholder="••••••••" {}
                      button type="button" aria-label="Show password" {
                        svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                          path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" {}
                          circle cx="12" cy="12" r="3" {}
                        }
                      }
                    }
                  }
                  div class="rem-for" {
                    div class="remember" {
                      input type="checkbox" name="remember" id="remember" {}
                      label for="remember" { "Remember me" }
                    }
                    a href="/forgot" { "Forgot password?" }
                  }
                  button type="submit" class="continue" {
                    "Sign in"
                    svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                      path d="M5 12h14" {}
                      path d="m12 5 7 7-7 7" {}
                    }
                  }
                }

                div id="g_id_onload" data-client_id=(&google_client_id().unwrap()) data-auto_prompt="false" data-callback="handleCredentialResponse" {}
                div class="g_id_signin" data-type="standard" data-size="large" data-theme="outline" data-text="sign_in_with" data-shape="rectangular" data-logo_alignment="left" {}

                div class="sub-button-text" {
                  p {
                    "Already have an account?"
                    button onclick="toggleLogin()" { "Sign in" }
                  }
                }
              }
            }
          }
          script src="https://accounts.google.com/gsi/client" async data-dynamic {}
          script type="text/javascript" data-dynamic {
            r#"
                    // Constant for the current login display (login vs. register)
                    let showLogin = true;

                    // Function to toggle between login and register
                    function toggleLogin() {
                        showLogin = !showLogin;

                        // Change the image in the left div to /static/images/signup.jpg
                        let img = document.querySelector('.left img');
                        img.src = showLogin ? '/static/images/signin.jpg' : '/static/images/signup.jpg';
                        // Update left title
                        let h1t = document.querySelector('.title h1');
                        h1t.innerText = showLogin ? 'Welcome Back' : 'Welcome!';
                        // Update left subtitle
                        let h2t = document.querySelector('.title h2');
                        h2t.innerText = showLogin ? 'We\'re so excited to see you again!' : 'A new world awaits you!';
                        // Update right form action
                        let form = document.querySelector('form');
                        form.action = showLogin ? '/api/login' : '/api/register';

                        // Update right form button text
                        let button = document.querySelector('.continue');
                        button.innerText = showLogin ? 'Sign in' : 'Sign up';
                        // Update sub button text
                        let subButton = document.querySelector('.sub-button-text button');
                        subButton.innerText = showLogin ? 'Sign up' : 'Sign in';
                        // Show/hide username field
                        let username = document.querySelector('.username');
                        username.style.display = showLogin ? 'none' : 'flex';
                    }

                    window.handleCredentialResponse = async function(response) {
                        if (!response || !response.credential) {
                            console.error('No response or credential');
                            return;
                        }

                        let credential = response.credential;
                        console.log(credential);

                        const res = await fetch('/api/login', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                credential: credential
                            }),
                        })

                        if (res.ok) {
                            window.location.reload();
                        } else {
                            console.error('Failed to login');
                        }
                    }
                    "#
          }
        }
      }
    }
}
