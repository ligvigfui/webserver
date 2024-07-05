const sections = document.querySelectorAll('section');

var nav = document.getElementsByTagName('nav')[0];
nav.innerHTML += "<a href='' onclick='scrollToSectionNumber(0); return false;'>Kezdőlap</a>";
for (var i = 1; i < sections.length; i++) {
    var text = sections[i].getElementsByTagName('h2')[0].innerHTML;
    nav.innerHTML += "<a href='' onclick='scrollToSectionNumber(" + i + "); return false;'>" + text + "</a>";
}

window.scrollTo({ top: 0, behavior: "smooth" });

function openTab(evt, tabName) {
	//get section__content class from parents
	var sectionId = evt.target.closest('.section__content').getElementsByTagName('h2')[0].innerHTML;
	var i, tabcontent, tablinks;
	tabcontent = document.getElementsByClassName("tabcontent");
	for (i = 0; i < tabcontent.length; i++) {
		if (tabcontent[i].closest('.section__content').getElementsByTagName('h2')[0].innerHTML != sectionId)
			continue;
		tabcontent[i].style.display = "none";
	}
	tablinks = document.getElementsByClassName("tablinks");
	for (i = 0; i < tablinks.length; i++) {
		if (tablinks[i].closest('.section__content').getElementsByTagName('h2')[0].innerHTML != sectionId)
			continue;
		tablinks[i].className = tablinks[i].className.replace(" active", "");
	}
	document.getElementById(tabName).style.display = "block";
	evt.currentTarget.className += " active";
}

// Get the elements with class="defaultOpen" and click on them
const defauls = document.getElementsByClassName("defaultOpen");
for (var i = 0; i < defauls.length; i++) {
	defauls[i].click();
}

/*
// listen to animation_toggle change
document.getElementById('animation_toggle').addEventListener('change', function() {
	if (this.checked) {
		// add event listeners
		window.addEventListener('wheel', wheel, { passive: false });
		window.addEventListener('touchmove', touchMove, { passive: false });
	}
	else {
		// remove event listeners if any
		window.removeEventListener('wheel', wheel, { passive: false });
		window.removeEventListener('touchmove', touchMove, { passive: false });
	}
});

let lastDeltaYs = [0, 0, 0];
function wheel(e) {
	console.log(e.deltaY);
    lastDeltaYs.shift();
	lastDeltaYs.push(e.deltaY)
    let currentSectionIndex = Math.round(window.scrollY / sectionHeight);
	// check if the user scrolled with a mouse wheel
    if ((lastDeltaYs.every((x) => x % 125 === 0)) ||
		(lastDeltaYs.every((x) => x % 20 === 0))) {
        e.preventDefault();
        if (e.deltaY > 0) {
            // Scrolling down
            if (currentSectionIndex < sections.length - 1) {
                window.scrollTo({ top: positions[currentSectionIndex + 1], behavior: 'smooth' });
            }
        } else {
            // Scrolling up
            if (currentSectionIndex > 0) {
                window.scrollTo({ top: positions[currentSectionIndex - 1], behavior: 'smooth' });
            }
        }
    }
}

let startY;
window.addEventListener('touchstart', function(e) {
	startY = e.touches[0].pageY;
}, false);

function touchMove(e) {
	e.preventDefault();
	let currentSectionIndex = Math.round(window.scrollY / sectionHeight);
	if (e.touches[0].pageY > startY) {
		// Scrolling up
		if (currentSectionIndex > 0) {
			window.scrollTo({ top: positions[currentSectionIndex - 1], behavior: 'smooth' });
		}
	} else {
		// Scrolling down
		if (currentSectionIndex < sections.length - 1) {
			window.scrollTo({ top: positions[currentSectionIndex + 1], behavior: 'smooth' });
		}
	}
}
*/
function scrollToSectionNumber(sectionIndex) {
	window.scrollTo({ top: sections[sectionIndex].offsetTop, behavior: 'smooth' });
	/*
	if (document.getElementById('animation_toggle').checked)
	{
	}
	else
	{
		window.scrollTo({ top: sections[sectionIndex].offsetTop, behavior: 'instant' });
	}*/
}
