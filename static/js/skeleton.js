
$("#frame-nav-search-input").keyup(function(event) {
    $.getJSON(
        "/api/search",
        { q: $("#frame-nav-search-input").val(), lang: $("html").attr("lang") },
        function(data) {
            var items = [];
            $.each(data.items, function(key, item) {
                items.push(
                    '<a href="' + item.url + '">' +
                    	item.title +
                    '</a>'
                )
            });
            $("#frame-nav-search-result").html(items.join(""));
            $("#frame-nav-search-result").addClass("show");

            document.addEventListener("click", function(ev) {
                $("#frame-nav-search-result").html("");
                $("#frame-nav-search-result").removeClass("show");
            });
        }
    )
});
