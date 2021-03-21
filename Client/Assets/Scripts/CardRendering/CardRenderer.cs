using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class CardRenderer : MonoBehaviour
{
    // Start is called before the first frame update
    public CardData Data;

    public GameObject[] Models;
    public Material[] Variants;
    private int _activeModel;

    public Animator Animator;

    public string Name;
    public TMPro.TMP_Text NameText;

    public Skill Weapons;
    public Skill Engineering;
    public Skill Biotech;
    public Skill Psychics;

    public Canvas SkillCanvas;

    public bool Hidden;


    [Range(1, 100)]
    public int SkillWeapons;

    [Range(1, 100)]
    public int SkillEngineering;

    [Range(1, 100)]
    public int SkillBiotech;

    [Range(1, 100)]
    public int SkillPsychics;

    public void DoUpdate()
    {
        SetVisuals();
        SetSkills();
    }

    private void Start()
    {
        SetSelected(false);
    }

    public void ReadToken(Token token)
    {
        Name = token.name;
        NameText.text = Name;
        SkillWeapons = token.weapons;
        SkillEngineering = token.engineering;
        SkillBiotech = token.biotech;
        SkillPsychics = token.psychics;
        DoUpdate();
    }

    private void SetModel(int newModel)
    {

        Models[_activeModel].SetActive(false);
        if (Hidden) return;
        Models[newModel].SetActive(true);
        _activeModel = newModel;
    }

    private void SetMaterial(int newVariant)
    {
        Models[_activeModel].GetComponent<SkinnedMeshRenderer>().material = Variants[newVariant];
    }

    public void SetSelected(bool selected)
    {
        Animator.SetBool("IsSelected", selected);
       
            LeanTween.scale(SkillCanvas.gameObject, selected? new Vector3(0.1f, 0.1f, 0.1f): Vector3.zero, 0.6f).setEase(LeanTweenType.easeInOutExpo);
        
    }


    private void SetVisuals()
    {
        NameText.text = Name;
        int modelId = (SkillWeapons * 3 + SkillEngineering * 2 + SkillBiotech + SkillPsychics) % Models.Length;
        int materialId = (SkillWeapons + SkillEngineering + SkillBiotech * 3 + SkillPsychics * 2) % Variants.Length;

        SetModel(modelId);
        SetMaterial(materialId);
    }

    private void SetSkills()
    {
        Weapons.Value.text = SkillWeapons.ToString();
        Engineering.Value.text = SkillEngineering.ToString();
        Biotech.Value.text = SkillBiotech.ToString();
        Psychics.Value.text = SkillPsychics.ToString();
    }
}
