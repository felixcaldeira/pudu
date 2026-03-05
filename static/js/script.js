// options: {
    // loadingText
    // beforeSubmit(formData)
    // onSuccess(response)
// }

// Usage example - with options:
// form.addEventListener('submit', async (e) => {
//     e.preventDefault();
//     await handleFormSubmit(form, {
//         loadingText: 'Speichern...',
//         beforeSubmit: (formData) => {
//             formData.set('content', editor.getMarkdown());
//         },
//         onSuccess: (response) => {
//             window.location.href = '/custom-redirect';
//         }
//     });
// });
const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];

async function handleFormSubmit(form, options = {}) {
    const errEl = $("error", form);
    const formData = new FormData(form);
    const action = form.getAttribute('action');
    const method = (form.getAttribute('method') || 'POST').toUpperCase();
    const submitBtn = $("button[type=submit]", form);
    const originalText = submitBtn.textContent;
    
    // Before submit callback
    if (options.beforeSubmit) {
        options.beforeSubmit(formData);
    }
    
    submitBtn.disabled = true;
    submitBtn.textContent = options.loadingText || '...';
    
    try {
        const response = await fetch(action, {
            method: method,
            body: formData
        });
        
        if (response.ok) {
            if (options.onSuccess) {
                options.onSuccess(response);
            } else if (response.redirected) {
                window.location.href = response.url;
            } else {
                window.location.reload();
            }
        } else {
            const errorText = await response.text();
            if (errEl) {
                errEl.classList.add("active");
                errEl.textContent = `Fehler (${response.status}): ${errorText || 'Unbekannter Fehler'}`;
            } else {
                alert(`Fehler (${response.status}): ${errorText || 'Unbekannter Fehler'}`);
            }
            submitBtn.disabled = false;
            submitBtn.textContent = originalText;
        }
    } catch (error) {
        console.error('Netzwerkfehler:', error);
        if (errEl) {
            errEl.classList.add("active");
            errEl.textContent = 'Netzwerkfehler: Bitte Internetverbindung prüfen.';
        } else {
            alert('Netzwerkfehler: Bitte Internetverbindung prüfen.');
        }
        submitBtn.disabled = false;
        submitBtn.textContent = originalText;
    }
}

function initQuillEditor(element, initialValue) {
    const editor = new Quill(element, {
        // theme: 'snow',
        // modules: {
        //     toolbar: [
        //         [{ 'header': [1, 2, 3, false] }],
        //         ['bold', 'italic'],
        //         [{ 'list': 'ordered'}, { 'list': 'bullet' }],
        //         ['link', 'code-block']
        //     ]
        // },
        theme: 'snow',
        placeholder: 'Inhalt hier schreiben...'
    });

    if (initialValue && initialValue !== '') {
        try {
            const delta = JSON.parse(initialValue);
            editor.setContents(delta);
        } catch (e) {
            editor.root.innerHTML = initialValue;
        }
    }
    
    return editor;
}

function getEditorContent(editor) {
    if (!editor) return null;    
    const delta = editor.getContents();
    return JSON.stringify(delta);
}

function setEditorContent(editor, content) {
    if (!editor) return;
    try {
        const delta = JSON.parse(content);
        editor.setContents(delta);
    } catch (e) {
        editor.root.innerHTML = content;
    }
}

/*
<!-- Display page -->
<link href="https://cdn.quilljs.com/1.3.6/quill.snow.css" rel="stylesheet">
<script src="https://cdn.quilljs.com/1.3.6/quill.js"></script>

<div id="content-display"></div>

<script>
    // Initialize read-only Quill instance
    const displayQuill = new Quill('#content-display', {
        theme: 'snow',
        readOnly: true,
        modules: {
            toolbar: false
        }
    });
    
    // Load and display content
    const savedContent = {{ section.content | safe }};
    
    try {
        // Parse Delta JSON
        const delta = JSON.parse(savedContent);
        displayQuill.setContents(delta);
    } catch (e) {
        console.error('Failed to parse content:', e);
    }
</script>

<style>
    #content-display .ql-container {
        border: none;
    }
</style>

<script src="https://cdn.jsdelivr.net/npm/quill-delta-to-html@0.12.1/dist/browser/QuillDeltaToHtmlConverter.bundle.js"></script>

<div id="content-display"></div>

<script>
    const savedContent = {{ section.content | safe }};
    
    try {
        const delta = JSON.parse(savedContent);
        const converter = new QuillDeltaToHtmlConverter(delta.ops, {});
        const html = converter.convert();
        document.getElementById('content-display').innerHTML = html;
    } catch (e) {
        console.error('Failed to render content:', e);
    }
</script>
*/