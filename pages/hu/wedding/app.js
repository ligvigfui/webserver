const positions = [];
const sections = document.querySelectorAll('section');
let sectionHeight = window.innerHeight;

function phoneVeiw() {
	var willDisapears = document.getElementsByClassName('willdisapear');
	var needsBackgroundImages = document.getElementsByClassName('needsbackgroundimage');
	if (window.innerWidth / window.innerHeight < 3 / 2 ) {
		// add the hidden property to the willdisapear class in css
		for (var i = 0; i < willDisapears.length; i++) {
			willDisapears[i].style.display = 'none';
		}
		for (var i = 1; i < needsBackgroundImages.length; i++) {
			// add bgi from the hidden > img.src

			// to .needsbackgroundimage > div
		}
	}
	else {
		// remove the hidden property from the willdisapear class in css
		for (var i = 0; i < willDisapears.length; i++) {
			willDisapears[i].style.display = 'flex';
		}
		for (var i = 1; i < needsBackgroundImages.length; i++) {
			// remove bgi from the hidden > img.src

			// from .needsbackgroundimage > div
		}
	}
}

function getPositions() {
	sectionHeight = window.innerHeight;
	positions.length = 0;
	for (let i = 0; i < sections.length; i++) {
		positions.push(i * sectionHeight);
	}
	scrollToSection(Math.round(window.scrollY / sectionHeight));
}

function onResponsive() {
	phoneVeiw();
	getPositions();
}

window.scrollTo({ top: 0, behavior: "smooth" });

var nav = document.getElementsByTagName('nav')[0];
nav.innerHTML += "<a href='' onclick='scrollToSection(0); return false;'>Kezdőlap</a>";
for (var i = 1; i < sections.length; i++) {
    var text = sections[i].getElementsByTagName('h2')[0].innerHTML;
    nav.innerHTML += "<a href='' onclick='scrollToSection(" + i + "); return false;'>" + text + "</a>";
}

onResponsive();
window.addEventListener('resize', onResponsive);
window.addEventListener('orientationchange', onResponsive);

function openTab(evt, tabName) {
	var i, tabcontent, tablinks;
	tabcontent = document.getElementsByClassName("tabcontent");
	for (i = 0; i < tabcontent.length; i++) {
		tabcontent[i].style.display = "none";
	}
	tablinks = document.getElementsByClassName("tablinks");
	for (i = 0; i < tablinks.length; i++) {
		tablinks[i].className = tablinks[i].className.replace(" active", "");
	}
	document.getElementById(tabName).style.display = "block";
	evt.currentTarget.className += " active";
}

// Get the element with id="defaultOpen" and click on it
document.getElementById("defaultOpen").click();

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

function scrollToSection(sectionIndex) {
	window.scrollTo({ top: positions[sectionIndex], behavior: 'smooth' });
	/*if (document.getElementById('animation_toggle').checked)
	{
	}
	else
	{
		window.scrollTo({ top: positions[sectionIndex], behavior: 'instant' });
	}*/
}