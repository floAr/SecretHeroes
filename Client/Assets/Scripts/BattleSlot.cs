using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BattleSlot : MonoBehaviour
{
    public bool isActive;

    public GameObject MainSystem;
    public ParticleSystem OrbSystem;
    public CardRenderer CharacterRenderer;
    public GameObject WinningBeam;
    public Light Spotlight;

    private void Start()
    {
        MainSystem.SetActive(false);
        CharacterRenderer.gameObject.SetActive(false);
        WinningBeam.SetActive(false);
        CharacterRenderer.IsSelected = false;
        Spotlight.enabled = false;
    }

    public void Activate(Token token)
    {
        isActive = true;
        MainSystem.SetActive(true);
        if (token != null) // we are looking at our character
        {
            OrbSystem.Stop();
            CharacterRenderer.gameObject.SetActive(true);
            CharacterRenderer.ReadToken(token);
            Spotlight.enabled = true;
        }
        else
        {

        }
    }

    private void Update()
    {
        if (isActive)
        {
            Spotlight.intensity = 3 + Mathf.Sin(Time.time) * 2f;
            if (!CharacterRenderer.IsSelected)
                CharacterRenderer.IsSelected=true;
        }
    }
}
