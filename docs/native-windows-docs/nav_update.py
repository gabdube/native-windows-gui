# Just a script that update the headers of every documentation page.

import glob, re
from os import path

APP_DIR = path.dirname(path.abspath(__file__))
NAV = r"""<!-- NAV BEGIN -->
<div id="nav">
    <button id="toggle-nav">
        <span class="sr-only">Toggle navigation</span>
        <span class="bar"></span>
        <span class="bar"></span>
        <span class="bar"></span>
    </button>
</div>

    
<div id='toc' class='mobile-hidden'>
<ul class='chapter'>
<li><a href='index.html'><b>1.</b> Introduction</a>
</li>
<li><a href='getting_started.html'><b>2.</b> Getting Started</a>
</li>

<li><a href="basics.html"><b>3.</b> Basics </a>
<ul class="section">
    <li><a href="controls.html"><b>3.1.</b> Controls </a></li>
    <li><a href="events.html"><b>3.2.</b> Events </a></li>
    <li><a href="helper.html"><b>3.3.</b> Helpers </a></li>
    <li><a href="small.html"><b>3.4.</b> Small application layout </a></li>
    <li><a href="limitations.html"><b>3.5.</b> Limitations </a></li>
    <li><a href="distribute.html"><b>3.6.</b> Distributing </a></li>
    <li><a href="features.html"><b>3.7.</b> Features </a></li>
</ul>
</li>

<li><a href="intermediate.html"><b>4.</b> Intermediate </a>
<ul class="section">
    <li><a href="layouts.html"><b>4.1.</b> Layouts </a></li>
    <li><a href="resources.html"><b>4.2.</b> Resources </a></li>
    <li><a href="dialogs.html"><b>4.3.</b> Dialogs </a></li>
    <li><a href="localization.html"><b>4.4.</b> Internationalization </a></li>
</ul>
</li>

<li><a href="advanced.html"><b>5.</b> Advanced </a>
<ul class="section">
    <li><a href="partial.html"><b>5.1.</b> Partials ui </a></li>
    <li><a href="dynamic_control.html"><b>5.2.</b> Dynamic control </a></li>
    <li><a href="dynamic_event.html"><b>5.3.</b> Dynamic events </a></li>
    <li><a href="multithreading.html"><b>5.4.</b> Multithreading </a></li>
</ul>
</li>

<li><a href="derive.html"><b>6.</b> Native-windows-derive </a>
<ul class="section">
    <li><a href="nwd_basics.html"><b>6.1.</b> Basics </a></li>
    <li><a href="nwd_controls.html"><b>6.1.</b> Controls </a></li>
    <li><a href="nwd_resources.html"><b>6.2.</b> Resources </a></li>
    <li><a href="nwd_events.html"><b>6.3.</b> Events </a></li>
    <li><a href="nwd_layouts.html"><b>6.4.</b> Layouts </a></li>
    <li><a href="nwd_partial.html"><b>6.5.</b> Partials </a></li>
</ul>
</li>

<li><a href="low.html"><b>7.</b> Low level stuff </a>
    <ul class="section">
        <li><a href="low_events.html"><b>7.1.</b> Raw event handling </a></li>
        <li><a href="extern_wrapping.html"><b>7.2.</b> Raw control handle </a></li>
    </ul>
</li>

</ul>
</div>
<!-- NAV END -->"""

NAV_BEGIN = "<!-- NAV BEGIN -->"
NAV_END = "<!-- NAV END -->"

headers_re = re.compile(f"{NAV_BEGIN}(.*?){NAV_END}", re.MULTILINE | re.DOTALL | re.ASCII)

for filename in glob.glob(APP_DIR+"/*.html"):
    with open(filename) as f:
        data = f.read()

    fixed = headers_re.sub(NAV, data)

    with open(filename, 'w') as f:
        f.write(fixed)
