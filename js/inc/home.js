
$("#home-search-input").keyup(function(event) {
    $.getJSON(
        "/api/search",
        { q: $("#home-search-input").val() },
        function(data) {
            var items = [];
            $.each(data.items, function(key, item) {
                items.push(
                    '<a href="' + item.url + '">' +
			'<i class="icon-type icon-type-' + item.type + '"></i>' +
                    	'<span class="here-title">' + item.title + '</span>' +
			'<span class="here-key">' + item.key + '</span>' +
                    '</a>'
                )
            });
            $("#home-search-result").html(items.join(""));
        }
    )
})

