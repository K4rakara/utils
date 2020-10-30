pub static CSS: &str = r#"
@import url("resource:///org/gtk/libgtk/theme/Adwaita/gtk-contained-dark.css");

@keyframes ripple {
    to {
        background-size: 1000% 1000%;
    }
}

rubberband {
    border: none;
    background: none;
}

scrollbar {
    border: none;
    background: none;
}

button {
    border: none;
    outline: none;
    background-color: #333333;
    background-size: 0% 0%;
    background-image: radial-gradient(circle, transparent 10%, transparent 0%);
    background-repeat: no-repeat;
    background-position: center;
    transition-property: background-size, background-image;
    transition-duration: .25s, .25s;
    transition-timing-function: ease-out;
    box-shadow: none;
}

button:hover {
    background-color: #3c3c3c;
    background-image: radial-gradient(circle, rgba(17, 17, 17, .12) 10%, transparent 0%);
}

button:active {
    background-color: #444444;
    background-size: 1000% 1000%;
    transition-duration: .25s, 0s;
    animation: ripple .25s ease-out forwards;
}

tooltip {
    border: none;
    border-radius: 16px;
}

tooltip.background {
    border: none;
    background-color: #111111;
}

#root {
    padding: 8px;
    background-color: #222222;
    border-color: #111111;
    border-width: 4px;
    border-radius: 16px;
    border-style: solid;
    font-family: 'Roboto';
}

#root #top #ctx {
    font-family: 'Fira Code';
}

#root #bot notebook#notebook {
    border: none;
    border-radius: 16px;
    background-color: #222222;
}

#root #bot notebook#notebook header {
    border-radius: 16px;
    background-color: #333333;
}

#root #bot notebook#notebook header tabs {
    border-radius: 16px;
    background-color: #333333;
}

#root #bot notebook#notebook header tabs tab {
    padding-top: 0;
    padding-bottom: 0;
    border: none;
}

#root #bot notebook#notebook header tabs tab:hover {
    background-color: #222222;
}

#root #bot notebook#notebook header tabs arrow {
    border: 16px;
    background: none;
}

#root #bot notebook#notebook stack {
    border-top-right-radius: 16px;
    border-bottom-right-radius: 16px;
    background-color: #222222;
}

#application-list #header-box #header {
    padding-left: 8px;
    padding-bottom: 4px;
}

#application-button {
    border: none;
    border-radius: 0;
    background-color: #222222;
    box-shadow: none;
    border-top-right-radius: 16px;
    border-bottom-right-radius: 16px;
}

#application-button:hover {
    background-color: #333333;
}

#application-button:active {
    background-color: #3c3c3c;
}

#action-button {
    border-radius: 50%;
}

#TRANSPARENT {
    border-radius: 1000000%;
    opacity: 0;
    background-color: rgba(0,0,0,0);
    background-size: 0;
}

#TRANSPARENT * {
    opacity: 0;
    background-color: rgba(0,0,0,0);
}
"#;
