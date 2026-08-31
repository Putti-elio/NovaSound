import 'htmx.org'
import './styles/app.css'

document.body.addEventListener('htmx:sendError', (event) => {
  const target = event.detail.target

  if (target?.id === 'artist-catalogue') {
    target.innerHTML = '<p class="status-message">Le catalogue est indisponible. Vérifiez que le backend Axum est démarré.</p>'
  }
})

document.body.addEventListener('htmx:responseError', (event) => {
  const target = event.detail.target

  if (target?.id === 'artist-catalogue') {
    target.innerHTML = '<p class="status-message">Le catalogue n’a pas pu être chargé.</p>'
  }
})
