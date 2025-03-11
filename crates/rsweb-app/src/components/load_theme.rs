use maud::PreEscaped;

pub const LOAD_THEME: PreEscaped<&'static str> = PreEscaped(
    "<script>(function(){const theme=localStorage.getItem('theme');if(theme){document.documentElement.setAttribute('data-theme', theme);}})()</script>",
);
